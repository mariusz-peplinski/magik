use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use code_core::auth::auth_for_stored_account;
use code_core::auth_accounts::{self, StoredAccount};
use code_core::config_types::AccountSwitchingMode;
use code_core::{AuthManager, ModelClient, Prompt, ResponseEvent};
use code_core::account_usage;
use code_core::config::Config;
use code_core::config_types::ReasoningEffort;
use code_core::debug_logger::DebugLogger;
use code_core::error::CodexErr;
use code_core::protocol::{Event, EventMsg, RateLimitSnapshotEvent, TokenCountEvent};
use code_login::AuthMode;
use code_protocol::models::{ContentItem, ResponseItem};
use chrono::Utc;
use futures::StreamExt;
use reqwest::StatusCode;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[cfg(feature = "code-fork")]
use crate::tui_event_extensions::handle_rate_limit;

use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use crate::thread_spawner;

const RATE_LIMIT_REFRESH_TIMEOUT: Duration = Duration::from_secs(45);

/// Fire-and-forget helper that refreshes rate limit data using a dedicated model
/// request. Results are funneled back into the main TUI loop via `AppEvent` so
/// history ordering stays consistent.
pub(super) fn start_rate_limit_refresh(
    app_event_tx: AppEventSender,
    config: Config,
    debug_enabled: bool,
) {
    start_rate_limit_refresh_with_options(
        app_event_tx,
        config,
        debug_enabled,
        None,
        true,
        true,
    );
}

pub(super) fn start_rate_limit_refresh_for_account(
    app_event_tx: AppEventSender,
    config: Config,
    debug_enabled: bool,
    account: StoredAccount,
    emit_ui: bool,
    notify_on_failure: bool,
) {
    start_rate_limit_refresh_with_options(
        app_event_tx,
        config,
        debug_enabled,
        Some(account),
        emit_ui,
        notify_on_failure,
    );
}

fn start_rate_limit_refresh_with_options(
    app_event_tx: AppEventSender,
    config: Config,
    debug_enabled: bool,
    account: Option<StoredAccount>,
    emit_ui: bool,
    notify_on_failure: bool,
) {
    let fallback_tx = app_event_tx.clone();
    if thread_spawner::spawn_lightweight("rate-refresh", move || {
        if let Err(err) = run_refresh(
            app_event_tx.clone(),
            config,
            debug_enabled,
            account,
            emit_ui,
        ) {
            if notify_on_failure {
                let message = format!("Failed to refresh rate limits: {err}");
                app_event_tx.send(AppEvent::RateLimitFetchFailed { message });
            } else {
                tracing::warn!("Failed to refresh rate limits: {err}");
            }
        }
    })
    .is_none()
    {
        if notify_on_failure {
            let message =
                "Failed to refresh rate limits: background worker unavailable".to_string();
            fallback_tx.send(AppEvent::RateLimitFetchFailed { message });
        } else {
            tracing::warn!("Failed to refresh rate limits: background worker unavailable");
        }
    }
}

fn run_refresh(
    app_event_tx: AppEventSender,
    config: Config,
    debug_enabled: bool,
    account: Option<StoredAccount>,
    emit_ui: bool,
) -> Result<()> {
    let started_at = Instant::now();
    let runtime = build_runtime()?;
    runtime.block_on(async move {
        let (auth_mgr, stored_account) = match account {
            Some(account) => {
                let auth = auth_for_stored_account(
                    &config.code_home,
                    &account,
                    &config.responses_originator_header,
                )
                .await
                .context("building auth for stored account")?;
                (
                    AuthManager::from_auth(
                        auth,
                        config.code_home.clone(),
                        config.responses_originator_header.clone(),
                    ),
                    Some(account),
                )
            }
            None => {
                let auth_mode = if config.using_chatgpt_auth {
                    AuthMode::ChatGPT
                } else {
                    AuthMode::ApiKey
                };
                (
                    AuthManager::shared_with_mode_and_originator(
                        config.code_home.clone(),
                        auth_mode,
                        config.responses_originator_header.clone(),
                    ),
                    None,
                )
            }
        };

        let (record_account_id, record_plan) = if let Some(account) = &stored_account {
            (
                Some(account.id.clone()),
                account
                    .tokens
                    .as_ref()
                    .and_then(|tokens| tokens.id_token.get_chatgpt_plan_type()),
            )
        } else {
            let active_id =
                auth_accounts::get_active_account_id(&config.code_home).ok().flatten();
            let account = active_id
                .as_deref()
                .and_then(|id| auth_accounts::find_account(&config.code_home, id).ok())
                .flatten();
            (
                active_id,
                account
                    .as_ref()
                    .and_then(|acc| acc.tokens.as_ref())
                    .and_then(|tokens| tokens.id_token.get_chatgpt_plan_type()),
            )
        };

        let account_id_for_logs = record_account_id.as_deref().unwrap_or("active-account");
        tracing::info!(
            account_id = account_id_for_logs,
            emit_ui,
            "rate limit refresh started"
        );

        let client = build_model_client(&config, auth_mgr, debug_enabled)?;

        let mut prompt = Prompt::default();
        prompt.store = false;
        prompt.user_instructions = config.user_instructions.clone();
        prompt.base_instructions_override = config.base_instructions.clone();
        prompt.input.push(ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "Yield immediately with only the message \"ok\"".to_string(),
            }],
            end_turn: None,
            phase: None,
        });
        prompt.set_log_tag("tui/rate_limit_refresh");

        let stream_opened_at = Instant::now();
        let stream_result = tokio::time::timeout(RATE_LIMIT_REFRESH_TIMEOUT, client.stream(&prompt))
            .await;
        let mut stream = match stream_result {
            Ok(Ok(stream)) => stream,
            Ok(Err(err)) => {
                maybe_record_auth_invalid_hint(
                    &config.code_home,
                    record_account_id.as_deref(),
                    record_plan.as_deref(),
                    &err,
                );
                maybe_emit_snapshot_stored(
                    &app_event_tx,
                    emit_ui,
                    record_account_id.as_deref(),
                );
                return Err(anyhow::Error::new(err)).context("requesting rate limit snapshot");
            }
            Err(_) => {
                tracing::warn!(
                    account_id = account_id_for_logs,
                    timeout_secs = RATE_LIMIT_REFRESH_TIMEOUT.as_secs(),
                    "rate limit refresh timed out before stream opened"
                );
                maybe_emit_snapshot_stored(
                    &app_event_tx,
                    emit_ui,
                    record_account_id.as_deref(),
                );
                return Err(anyhow::anyhow!(
                    "requesting rate limit snapshot timed out after {}s",
                    RATE_LIMIT_REFRESH_TIMEOUT.as_secs()
                ));
            }
        };

        tracing::info!(
            account_id = account_id_for_logs,
            elapsed_ms = stream_opened_at.elapsed().as_millis() as u64,
            "rate limit refresh stream opened"
        );

        let stream_read_started_at = Instant::now();
        let (snapshot, events_seen) = tokio::time::timeout(RATE_LIMIT_REFRESH_TIMEOUT, async {
            let mut snapshot = None;
            let mut events_seen = 0usize;
            while let Some(event) = stream.next().await {
                events_seen = events_seen.saturating_add(1);
                match event.context("reading rate limit refresh stream")? {
                    ResponseEvent::RateLimits(s) => {
                        snapshot = Some(s);
                        break;
                    }
                    ResponseEvent::Completed { .. } => break,
                    _ => {}
                }
            }
            Ok::<(Option<_>, usize), anyhow::Error>((snapshot, events_seen))
        })
        .await
        .map_err(|_| {
            tracing::warn!(
                account_id = account_id_for_logs,
                timeout_secs = RATE_LIMIT_REFRESH_TIMEOUT.as_secs(),
                "rate limit refresh timed out while waiting for stream events"
            );
            maybe_emit_snapshot_stored(&app_event_tx, emit_ui, record_account_id.as_deref());
            anyhow::anyhow!(
                "waiting for rate limit snapshot timed out after {}s",
                RATE_LIMIT_REFRESH_TIMEOUT.as_secs()
            )
        })??;

        let proto_snapshot = snapshot.context("rate limit snapshot missing from response")?;

        let snapshot: RateLimitSnapshotEvent = proto_snapshot.clone();

        if let Some(account_id) = record_account_id.as_deref() {
            if let Err(err) = account_usage::record_rate_limit_snapshot(
                &config.code_home,
                account_id,
                record_plan.as_deref(),
                &snapshot,
                Utc::now(),
            ) {
                tracing::warn!("Failed to persist rate limit snapshot: {err}");
            }
        }

        #[cfg(feature = "code-fork")]
        handle_rate_limit(&snapshot, &app_event_tx);

        if emit_ui {
            let event = Event {
                id: "rate-limit-refresh".to_string(),
                event_seq: 0,
                msg: EventMsg::TokenCount(TokenCountEvent {
                    info: None,
                    rate_limits: Some(snapshot),
                }),
                order: None,
            };

            app_event_tx.send(AppEvent::CodexEvent(event));
        } else if let Some(account_id) = record_account_id.as_ref() {
            app_event_tx.send(AppEvent::RateLimitSnapshotStored {
                account_id: account_id.clone(),
            });
        }

        tracing::info!(
            account_id = account_id_for_logs,
            events_seen,
            stream_elapsed_ms = stream_read_started_at.elapsed().as_millis() as u64,
            total_elapsed_ms = started_at.elapsed().as_millis() as u64,
            "rate limit refresh completed"
        );
        Ok(())
    })
}

fn maybe_emit_snapshot_stored(app_event_tx: &AppEventSender, emit_ui: bool, account_id: Option<&str>) {
    if emit_ui {
        return;
    }

    if let Some(account_id) = account_id {
        app_event_tx.send(AppEvent::RateLimitSnapshotStored {
            account_id: account_id.to_string(),
        });
    }
}

fn build_runtime() -> Result<Runtime> {
    Ok(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .context("building rate limit refresh runtime")?,
    )
}

fn build_model_client(
    config: &Config,
    auth_mgr: Arc<AuthManager>,
    debug_enabled: bool,
) -> Result<ModelClient> {
    let debug_logger = DebugLogger::new(debug_enabled)
        .or_else(|_| DebugLogger::new(false))
        .context("initializing debug logger")?;

    let mut refresh_config = config.clone();
    // Rate-limit snapshot refresh should never mutate global account selection.
    refresh_config.auto_switch_accounts_on_rate_limit = false;
    refresh_config.account_switching_mode = AccountSwitchingMode::Manual;

    let client = ModelClient::new(
        Arc::new(refresh_config),
        Some(auth_mgr),
        None,
        config.model_provider.clone(),
        ReasoningEffort::Low,
        config.model_reasoning_summary,
        config.model_text_verbosity,
        Uuid::new_v4(),
        Arc::new(Mutex::new(debug_logger)),
    );

    Ok(client)
}

fn maybe_record_auth_invalid_hint(
    code_home: &std::path::Path,
    account_id: Option<&str>,
    plan: Option<&str>,
    err: &CodexErr,
) {
    let Some(account_id) = account_id else {
        return;
    };

    let should_mark = match err {
        CodexErr::UnexpectedStatus(unexpected) => unexpected.status == StatusCode::UNAUTHORIZED,
        CodexErr::AuthRefreshPermanent(_) => true,
        _ => false,
    };
    if !should_mark {
        return;
    }

    let observed_at = Utc::now();
    if let Err(err) = account_usage::record_auth_invalid_hint(code_home, account_id, plan, observed_at) {
        tracing::warn!("Failed to persist auth invalid hint: {err}");
    }
}
