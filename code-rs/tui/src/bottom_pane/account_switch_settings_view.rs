use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use crate::colors;
use crossterm::event::{KeyCode, KeyEvent};
use code_core::config_types::AccountSwitchingMode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

pub(crate) struct AccountSwitchSettingsView {
    app_event_tx: AppEventSender,
    selected_index: usize,
    auto_switch_enabled: bool,
    api_key_fallback_enabled: bool,
    switching_mode: AccountSwitchingMode,
    is_complete: bool,
}

impl AccountSwitchSettingsView {
    pub(crate) fn new(
        app_event_tx: AppEventSender,
        auto_switch_enabled: bool,
        api_key_fallback_enabled: bool,
        switching_mode: AccountSwitchingMode,
    ) -> Self {
        Self {
            app_event_tx,
            selected_index: 0,
            auto_switch_enabled,
            api_key_fallback_enabled,
            switching_mode,
            is_complete: false,
        }
    }

    fn option_count() -> usize {
        4
    }

    fn toggle_auto_switch(&mut self) {
        self.auto_switch_enabled = !self.auto_switch_enabled;
        self.app_event_tx
            .send(AppEvent::SetAutoSwitchAccountsOnRateLimit(
                self.auto_switch_enabled,
            ));
    }

    fn toggle_api_key_fallback(&mut self) {
        self.api_key_fallback_enabled = !self.api_key_fallback_enabled;
        self.app_event_tx
            .send(AppEvent::SetApiKeyFallbackOnAllAccountsLimited(
                self.api_key_fallback_enabled,
            ));
    }

    fn close(&mut self) {
        self.is_complete = true;
    }

    fn switching_mode_label(mode: AccountSwitchingMode) -> &'static str {
        match mode {
            AccountSwitchingMode::Manual => "Manual",
            AccountSwitchingMode::OnLimit => "On limit",
            AccountSwitchingMode::EvenUsage => "Even usage",
            AccountSwitchingMode::Step45 => "Step 45%",
            AccountSwitchingMode::ResetBased => "Reset-based",
        }
    }

    fn switching_mode_description(mode: AccountSwitchingMode) -> &'static str {
        match mode {
            AccountSwitchingMode::Manual => {
                "Never auto-switch accounts; stick to the currently active account."
            }
            AccountSwitchingMode::OnLimit => "Only switches when the active account is limited.",
            AccountSwitchingMode::EvenUsage => "Keeps accounts within ~10% usage of each other.",
            AccountSwitchingMode::Step45 => "Rotates when an account crosses 45% steps.",
            AccountSwitchingMode::ResetBased => {
                "Prefers accounts with the soonest short-window reset time."
            }
        }
    }

    fn next_switching_mode(mode: AccountSwitchingMode) -> AccountSwitchingMode {
        match mode {
            AccountSwitchingMode::Manual => AccountSwitchingMode::OnLimit,
            AccountSwitchingMode::OnLimit => AccountSwitchingMode::EvenUsage,
            AccountSwitchingMode::EvenUsage => AccountSwitchingMode::Step45,
            AccountSwitchingMode::Step45 => AccountSwitchingMode::ResetBased,
            AccountSwitchingMode::ResetBased => AccountSwitchingMode::Manual,
        }
    }

    fn cycle_switching_mode(&mut self) {
        self.switching_mode = Self::next_switching_mode(self.switching_mode);
        self.app_event_tx
            .send(AppEvent::SetAccountSwitchingMode(self.switching_mode));
    }

    fn activate_selected(&mut self) {
        match self.selected_index {
            0 => self.toggle_auto_switch(),
            1 => self.toggle_api_key_fallback(),
            2 => self.cycle_switching_mode(),
            3 => self.close(),
            _ => {}
        }
    }

    fn info_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        lines.push(Line::from(vec![Span::styled(
            "Accounts",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        let highlight = Style::default()
            .fg(colors::primary())
            .add_modifier(Modifier::BOLD);
        let normal = Style::default().fg(colors::text());
        let dim = Style::default().fg(colors::text_dim());

        let row_toggle = |idx: usize, label: &str, enabled: bool| -> Line<'static> {
            let selected = idx == self.selected_index;
            let indicator = if selected { ">" } else { " " };
            let style = if selected { highlight } else { normal };
            let state_style = if enabled {
                Style::default().fg(colors::success())
            } else {
                Style::default().fg(colors::text_dim())
            };
            Line::from(vec![
                Span::styled(format!("{indicator} "), style),
                Span::styled(label.to_string(), style),
                Span::raw("  "),
                Span::styled(
                    format!("[{}]", if enabled { "x" } else { " " }),
                    state_style,
                ),
            ])
        };

        let row_mode = |idx: usize, label: &str, value: &str| -> Line<'static> {
            let selected = idx == self.selected_index;
            let indicator = if selected { ">" } else { " " };
            let style = if selected { highlight } else { normal };
            Line::from(vec![
                Span::styled(format!("{indicator} "), style),
                Span::styled(label.to_string(), style),
                Span::raw("  "),
                Span::styled(
                    format!("[{value}]"),
                    Style::default().fg(colors::info()),
                ),
            ])
        };

        lines.push(row_toggle(
            0,
            "Auto-switch on rate/usage limit",
            self.auto_switch_enabled,
        ));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                "Switches to another connected account on 429/usage_limit.",
                dim,
            ),
        ]));

        lines.push(row_toggle(
            1,
            "API key fallback when all accounts limited",
            self.api_key_fallback_enabled,
        ));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                "Only used if every connected ChatGPT account is limited.",
                dim,
            ),
        ]));

        lines.push(row_mode(
            2,
            "Switching strategy",
            Self::switching_mode_label(self.switching_mode),
        ));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(Self::switching_mode_description(self.switching_mode), dim),
        ]));

        lines.push(Line::from(""));

        let close_selected = self.selected_index == 3;
        let close_style = if close_selected { highlight } else { normal };
        let indicator = if close_selected { ">" } else { " " };
        lines.push(Line::from(vec![
            Span::styled(format!("{indicator} "), close_style),
            Span::styled("Close", close_style),
        ]));

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(" Up/Down", Style::default().fg(colors::function())),
            Span::styled(" Navigate  ", dim),
            Span::styled("Enter", Style::default().fg(colors::success())),
            Span::styled(" Toggle  ", dim),
            Span::styled("Esc", Style::default().fg(colors::error())),
            Span::styled(" Close", dim),
        ]));

        lines
    }

    pub(crate) fn render_without_frame(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        Paragraph::new(self.info_lines())
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(colors::background()).fg(colors::text()))
            .render(area, buf);
    }

    pub(crate) fn handle_key_event_direct(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.close(),
            KeyCode::Up => {
                self.selected_index = self.selected_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Tab => {
                self.selected_index = (self.selected_index + 1) % Self::option_count();
            }
            KeyCode::BackTab => {
                if self.selected_index == 0 {
                    self.selected_index = Self::option_count() - 1;
                } else {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => self.activate_selected(),
            _ => {}
        }
    }

    pub(crate) fn is_view_complete(&self) -> bool {
        self.is_complete
    }
}
