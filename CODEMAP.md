# CODEMAP

This file is a high-signal navigation map for coding agents working in this
repository.

Scope:
- Covers where code lives, what each area owns, and where to start for common
  changes.
- Prioritizes `code-rs/` (active Rust workspace) over `codex-rs/` (upstream
  mirror).

## Tag Legend

- `runtime`: user-facing execution path
- `entrypoint`: executable binary or primary command surface
- `protocol`: shared types / wire contracts
- `ui`: terminal UI or rendering layer
- `orchestration`: agent coordination / task routing
- `integration`: external systems (MCP, browser, cloud, auth)
- `tooling`: build/release/dev automation
- `animation`: motion and visual transition logic
- `mirror`: sync-only copy from upstream (do not edit for local features)
- `tests`: primary regression coverage location

## Top-Level Layout

| Path | Responsibility | Tags |
|---|---|---|
| `README.md` | Project overview, install, command quickstart, feature highlights | `runtime` |
| `AGENTS.md` | Agent operating rules for this repo (also hardlinked as `agents.md`) | `tooling` |
| `CODEMAP.md` | This navigation map for coding agents | `tooling` |
| `build-fast.sh` | Required validation script before completion; canonical build check | `tooling` |
| `pre-release.sh` | Preflight for main-branch release readiness | `tooling` |
| `docs/` | User/developer docs (config, commands, sandbox, TUI behavior) | `tooling` |
| `.github/workflows/` | CI/release automation (`rust-ci`, `release`, upstream merge) | `tooling` |
| `scripts/` | Repo automation scripts (release notes check, GH run waiter, proxy helpers) | `tooling` |
| `code-rs/` | Active Rust workspace for this fork | `runtime` |
| `codex-rs/` | Upstream mirror of `openai/codex`; reference-only in this repo | `mirror` |
| `codex-cli/` | NPM package wrapper that ships platform binaries and launcher scripts | `entrypoint` |
| `sdk/typescript/` | TypeScript SDK for driving Code/Codex workflows programmatically | `integration` |
| `shell-tool-mcp/` | MCP server package focused on shell execution tooling | `integration` |
| `third_party/` | Vendored/third-party assets (for example wezterm bits) | `tooling` |

## Rust Workspace (`code-rs/`)

### High-impact crates (start here first)

| Path | Crate | Responsibility | Tags |
|---|---|---|---|
| `code-rs/core/` | `code-core` | Main business logic: config loading, conversation/session orchestration, tools, safety, model/provider wiring, state/history | `runtime`, `orchestration` |
| `code-rs/tui/` | `code-tui` | Full-screen Ratatui app, chat widget, history rendering, overlays/modals, approvals, slash-command UX | `ui`, `runtime` |
| `code-rs/exec/` | `code-exec` | Headless/non-interactive runtime (`code exec`), JSON/human event output, auto-drive execution path | `entrypoint`, `runtime` |
| `code-rs/cli/` | `code-cli` | Multi-tool CLI entrypoint (`code` command), command routing for TUI/exec/mcp/login/cloud/etc. | `entrypoint`, `orchestration` |
| `code-rs/protocol/` | `code-protocol` | Shared protocol and schema-like types used across core, TUI, and servers | `protocol` |
| `code-rs/common/` | `code-common` | Shared utilities (CLI arg enums, model presets, config summary helpers) | `tooling` |

### Integration and feature crates

| Path | Crate | Responsibility | Tags |
|---|---|---|---|
| `code-rs/mcp-server/` | `code-mcp-server` | MCP server runtime and tool handlers | `integration`, `entrypoint` |
| `code-rs/mcp-client/` | `code-mcp-client` | MCP client wrapper used by core/runtime | `integration` |
| `code-rs/mcp-types/` | `code-mcp-types` | Re-exported upstream MCP types for wire compatibility | `protocol` |
| `code-rs/browser/` | `code-browser` | Browser automation manager, page/session handling, browser tool schema | `integration` |
| `code-rs/cloud-tasks/` | `code-cloud-tasks` | Cloud task UI/CLI flow and local apply workflow | `integration`, `ui` |
| `code-rs/cloud-tasks-client/` | `code-cloud-tasks-client` | Backend API abstraction for cloud task operations | `integration` |
| `code-rs/code-auto-drive-core/` | `code-auto-drive-core` | Auto Drive coordination engine, retry/session metrics/history | `orchestration` |
| `code-rs/code-auto-drive-diagnostics/` | `code-auto-drive-diagnostics` | Auto Drive completion verification/diagnostics layer | `orchestration` |
| `code-rs/backend-client/` | `code-backend-client` | Typed client for backend task/config APIs | `integration` |
| `code-rs/login/` | `code-login` | Device code/ChatGPT login flows and auth server helpers | `integration` |
| `code-rs/ollama/` | `code-ollama` | OSS/local-model bootstrap and Ollama integration | `integration` |
| `code-rs/responses-api-proxy/` | `code-responses-api-proxy` | Restricted local proxy forwarding only `/v1/responses` | `integration`, `entrypoint` |
| `code-rs/file-search/` | `code-file-search` | Fast fuzzy file search used by `@` workflows | `runtime` |
| `code-rs/execpolicy/` | `code-execpolicy` | Starlark-based exec policy parsing/checking for command controls | `runtime` |
| `code-rs/git-tooling/` | `code-git-tooling` | Git helpers (ghost commits/snapshot restore/symlink utils) | `tooling` |
| `code-rs/git-apply/` | `code-git-apply` | Structured wrapper around `git apply` behavior/results | `tooling` |
| `code-rs/apply-patch/` | `code-apply-patch` | Internal apply_patch parser and patch application engine | `tooling` |
| `code-rs/linux-sandbox/` | `code-linux-sandbox` | Linux sandbox entrypoint and landlock run logic | `runtime`, `entrypoint` |
| `code-rs/process-hardening/` | `code-process-hardening` | Pre-main hardening hooks (env stripping, core dump/ptrace restrictions) | `runtime` |

### Utility and support crates

| Path | Crate | Responsibility | Tags |
|---|---|---|---|
| `code-rs/app-server/` | `code-app-server` | App server executable/runtime helpers | `integration`, `entrypoint` |
| `code-rs/app-server-protocol/` | `code-app-server-protocol` | Compatibility shim that re-exports MCP protocol surface | `protocol` |
| `code-rs/chatgpt/` | `code-chatgpt` | First-party ChatGPT/Codex API-related helpers and commands | `integration` |
| `code-rs/ansi-escape/` | `code-ansi-escape` | ANSI-to-Ratatui conversion helpers | `ui` |
| `code-rs/arg0/` | `code-arg0` | Arg0 dispatch for multi-binary behavior from a single executable | `entrypoint` |
| `code-rs/otel/` | `code-otel` | Optional OpenTelemetry provider/config glue | `integration` |
| `code-rs/rmcp-client/` | `code-rmcp-client` | Alternative MCP client implementation helpers | `integration` |
| `code-rs/protocol-ts/` | `code-protocol-ts` | TypeScript binding generation for protocol types | `protocol`, `tooling` |
| `code-rs/code-backend-openapi-models/` | `code-backend-openapi-models` | Generated OpenAPI model re-exports | `protocol` |
| `code-rs/code-version/` | `code-version` | Build/version string and wire-compat helpers | `tooling` |
| `code-rs/utils/json-to-toml/` | `code-utils-json-to-toml` | JSON/TOML conversion utility | `tooling` |
| `code-rs/utils/readiness/` | `code-utils-readiness` | readiness checking helper utility | `tooling` |

## Key Entry Points

| File | What it controls | Tags |
|---|---|---|
| `code-rs/cli/src/main.rs` | Top-level `code` command parser and subcommand dispatch | `entrypoint` |
| `code-rs/tui/src/main.rs` | Interactive TUI process boot | `entrypoint`, `ui` |
| `code-rs/exec/src/main.rs` | Headless `exec` process boot | `entrypoint` |
| `code-rs/core/src/codex.rs` | Core session engine and event production | `runtime`, `orchestration` |
| `code-rs/tui/src/chatwidget.rs` | Main chat controller for event handling/render state | `ui`, `runtime` |
| `code-rs/tui/src/history_cell/` | Rendering primitives for transcript/history cards | `ui` |
| `code-rs/mcp-server/src/lib.rs` | MCP server loop and request handling glue | `integration` |
| `code-rs/browser/src/tools/browser_tools.rs` | Browser tool contract and invocation glue | `integration` |

## TUI Visual Architecture

### Screen composition (how the full UI is built)

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/tui/src/tui.rs` | Terminal lifecycle (enter/leave alt-screen, clear behavior, terminal capability checks) | `ui`, `runtime` |
| `code-rs/tui/src/app/state.rs` | Global app state, redraw scheduling/debouncing, frame timers, input thread state | `ui` |
| `code-rs/tui/src/app/render.rs` | Frame draw loop (`draw_next_frame`) and top-level app render dispatch | `ui` |
| `code-rs/tui/src/app/init.rs` | App bootstrap: onboarding-vs-chat state, input/event threads, startup wiring | `ui`, `runtime` |
| `code-rs/tui/src/chatwidget.rs` | Main chat screen widget; owns status bar, history viewport, bottom pane composition | `ui`, `runtime` |
| `code-rs/tui/src/chatwidget/layout_scroll.rs` | Vertical layout partitioning and scroll behavior (history vs bottom pane) | `ui` |
| `code-rs/tui/src/height_manager.rs` | Dynamic height policy for status/history/bottom pane under changing terminal sizes | `ui` |

### Header / status bar

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/tui/src/chatwidget.rs` (`render_status_bar`) | Top header content (`Every Code`, model, reasoning, directory, branch) and width-based elision | `ui` |
| `code-rs/tui/src/header_wave.rs` | Animated spectral header stripe effect and frame cadence | `ui`, `animation` |
| `code-rs/tui/src/chatwidget/session_header.rs` | Session header model text abstraction | `ui` |

Notes:
- Header elision priority in tight widths: reasoning, then model, then branch, then directory.
- Header animation is gated and scheduled via `AppEvent::ScheduleFrameIn`.

### Bottom pane / composer / footer hints

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/tui/src/bottom_pane/mod.rs` | Bottom pane container, active-view vs composer switching, height policy | `ui` |
| `code-rs/tui/src/bottom_pane/chat_composer.rs` | Input field behavior, placeholder, footer hints/notices, slash popup integration, submission | `ui`, `runtime` |
| `code-rs/tui/src/bottom_pane/chat_composer_history.rs` | Composer history navigation and metadata-backed retrieval | `ui` |
| `code-rs/tui/src/bottom_pane/bottom_pane_view.rs` | Interface for non-composer modal/panel views rendered in bottom pane | `ui` |
| `code-rs/tui/src/bottom_pane/*_view.rs` | Settings/editor overlays (agents, MCP, theme picker, review, validation, etc.) | `ui` |

Notes:
- Footer text/hints are primarily composed in `ChatComposer::render_footer`.
- Auto Drive and Auto Review footer states are surfaced through composer footer sections.

### Message history and formatting

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/tui/src/chatwidget/history_render.rs` | History virtualization, cached cell layout, prefix sums, visible slice computation | `ui` |
| `code-rs/tui/src/history_cell/mod.rs` | Registry for history cell types and rendering helpers | `ui` |
| `code-rs/tui/src/history_cell/assistant.rs` | Assistant markdown cells and assistant-specific layout cache | `ui` |
| `code-rs/tui/src/history_cell/plain.rs` | Plain user/assistant/system lines and simple text cells | `ui` |
| `code-rs/tui/src/history_cell/exec.rs` | Running/completed command cells and command output presentation | `ui` |
| `code-rs/tui/src/history_cell/diff.rs` | Diff cell rendering and diff summaries | `ui` |
| `code-rs/tui/src/history_cell/reasoning.rs` | Collapsible reasoning cell rendering | `ui` |
| `code-rs/tui/src/history_cell/tool*.rs` | Tool invocation/result cell rendering (MCP/browser/custom/web-fetch/etc.) | `ui` |
| `code-rs/tui/src/markdown_render.rs` | Markdown-to-Ratatui rendering path used by assistant outputs | `ui` |
| `code-rs/tui/src/markdown_renderer.rs` | Custom markdown rendering pipeline (headings/lists/code blocks/tables/callouts) | `ui` |
| `code-rs/tui/src/syntax_highlight.rs` | Syntax highlighting themes/rules for fenced code blocks and diffs | `ui` |

### Theming and visual language

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/core/src/config_types.rs` (`ThemeConfig`, `ThemeName`, `ThemeColors`) | Theme configuration schema, built-in theme catalog names, custom color fields | `protocol`, `ui` |
| `code-rs/core/src/theme_files.rs` | File-backed theme catalog storage/loading (`<code_home>/themes`) and TOML serialization | `runtime`, `ui` |
| `code-rs/tui/src/theme.rs` | Theme state, built-in theme resolution, custom overrides, ANSI16/ANSI256 mapping | `ui` |
| `code-rs/tui/src/colors.rs` | Canonical color getters consumed across widgets | `ui` |
| `code-rs/tui/src/bottom_pane/theme_selection_view.rs` | Theme picker UI in settings overlay | `ui` |
| `code-rs/tui/src/card_theme.rs` | Card-style gradients/reveal styles used by visual cards | `ui`, `animation` |
| `code-rs/tui/src/gradient_background.rs` | Gradient/reveal renderer for animated card backgrounds | `ui`, `animation` |
| `code-rs/tui/src/spinner.rs` | Spinner definitions and spinner-style selection | `ui`, `animation` |

Theme catalog currently includes light, dark, ANSI16, and custom variants:
- Light: `light-photon`, `light-prism-rainbow`, `light-vivid-triad`, `light-porcelain`, `light-sandbar`, `light-glacier`
- Dark: `dark-carbon-night`, `dark-shinobi-dusk`, `dark-oled-black-pro`, `dark-amber-terminal`, `dark-aurora-flux`, `dark-charcoal-rainbow`, `dark-zen-garden`, `dark-paper-light-pro`
- ANSI16 fallbacks: `light-photon-ansi16`, `dark-carbon-ansi16`
- Custom: `custom`

Theme storage notes:
- Theme files are loaded from `<code_home>/themes` (default `~/.magic/themes`).
- `code_home` resolution order is `CODE_HOME`, then `CODEX_HOME`, then `~/.magic`.
- With no env override, read paths can fall back to legacy `~/.codex` when a default-path file is missing; writes target `<code_home>`.

### Startup/welcome and motion effects

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/tui/src/greeting.rs` | Time-aware startup greeting/placeholder text | `ui` |
| `code-rs/tui/src/chatwidget.rs` (`check_for_initial_animations`) | Kicks initial animation frames when animating cells exist | `ui`, `animation` |
| `code-rs/tui/src/chatwidget.rs` (`seed_test_mode_greeting`) | Test-mode greeting prelude cells | `ui`, `tests` |
| `code-rs/tui/src/glitch_animation.rs` | Shared color-mixing and gradient helpers for visual effects | `ui`, `animation` |
| `code-rs/tui/src/shimmer.rs` | Shimmer/animated visual helper logic | `ui`, `animation` |

### Alternate-screen vs standard-terminal mode

| Path | Responsibility | Tags |
|---|---|---|
| `code-rs/tui/src/tui.rs` | Alternate screen entry/exit policy and terminal setup behavior | `ui`, `runtime` |
| `code-rs/tui/src/insert_history.rs` | Standard-terminal mode: inserts chat lines into terminal scrollback safely | `ui`, `runtime` |
| `docs/tui-alternate-screen.md` | Rationale and behavior details for alt-screen handling | `tooling`, `ui` |

## UI Docs Index

| Path | Focus |
|---|---|
| `docs/settings.md` | Settings overlay behavior and sections |
| `docs/tui-chat-composer.md` | Composer state machine, paste burst logic, slash behavior |
| `docs/tui-request-user-input.md` | Request-user-input overlay layout/focus model |
| `docs/tui-alternate-screen.md` | Fullscreen alternate-screen strategy |
| `docs/tui-stream-chunking-review.md` | Streaming chunking and animation interactions |
| `docs/tui-stream-chunking-tuning.md` | Tuning notes for stream chunking behavior |
| `docs/tui-stream-chunking-validation.md` | Validation checks for streaming chunking changes |

## UI Change Cookbook

Use this as a quick "where do I edit" index for common UI work.

| Goal | Primary edit points | Also check |
|---|---|---|
| Change startup greeting text or time-of-day phrasing | `code-rs/tui/src/greeting.rs` | `code-rs/tui/src/bottom_pane/chat_composer.rs` (placeholder rendering) |
| Change startup intro/prelude cells shown in test flows | `code-rs/tui/src/chatwidget.rs` (`seed_test_mode_greeting`) | `code-rs/tui/src/history_cell/assistant.rs` |
| Change top header contents (model/reasoning/dir/branch) | `code-rs/tui/src/chatwidget.rs` (`render_status_bar`) | `code-rs/tui/src/chatwidget/session_header.rs` |
| Change header animation visuals | `code-rs/tui/src/header_wave.rs` | `code-rs/tui/src/chatwidget.rs` (frame scheduling + render call) |
| Change whole-screen layout split (header/history/footer sizes) | `code-rs/tui/src/chatwidget/layout_scroll.rs` | `code-rs/tui/src/height_manager.rs`, `code-rs/tui/src/app/render.rs` |
| Change composer input field visuals (border/padding/title) | `code-rs/tui/src/bottom_pane/chat_composer.rs` (`render_ref`) | `code-rs/tui/src/layout_consts.rs`, `code-rs/tui/src/ui_consts.rs` |
| Change footer hints / status text in input area | `code-rs/tui/src/bottom_pane/chat_composer.rs` (`render_footer`) | `code-rs/tui/src/chatwidget.rs` (places that set notices/hints) |
| Change chat message card/bubble style globally | `code-rs/tui/src/history_cell/core.rs` and `code-rs/tui/src/history_cell/card_style.rs` | `code-rs/tui/src/history_cell/mod.rs` |
| Change assistant markdown rendering/spacing rules | `code-rs/tui/src/markdown_render.rs` and `code-rs/tui/src/markdown_renderer.rs` | `code-rs/tui/src/history_cell/assistant.rs` |
| Change code block syntax colors or theme mapping | `code-rs/tui/src/syntax_highlight.rs` | `code-rs/tui/src/theme.rs` |
| Change exec/command cell formatting | `code-rs/tui/src/history_cell/exec.rs` | `code-rs/tui/src/history_cell/exec_helpers.rs`, `code-rs/tui/src/chatwidget.rs` |
| Change diff cell appearance | `code-rs/tui/src/history_cell/diff.rs` | `code-rs/tui/src/diff_render.rs` |
| Change reasoning cell presentation | `code-rs/tui/src/history_cell/reasoning.rs` | `code-rs/tui/src/chatwidget.rs` (expand/collapse state and insertion) |
| Change tool call cards (browser/MCP/custom/web fetch) | `code-rs/tui/src/history_cell/tool_factory.rs` | `code-rs/tui/src/history_cell/tool.rs`, `code-rs/tui/src/chatwidget/tools.rs` |
| Change theme palette values or add theme variants | `code-rs/tui/src/theme.rs` | `code-rs/core/src/config_types.rs` (`ThemeName`) |
| Change color tokens used everywhere (`text`, `border`, `success`, etc.) | `code-rs/tui/src/colors.rs` | `code-rs/tui/src/theme.rs` |
| Change theme picker behavior | `code-rs/tui/src/bottom_pane/theme_selection_view.rs` | `code-rs/tui/src/bottom_pane/settings_overlay.rs` |
| Change spinner set / spinner animation cadence | `code-rs/tui/src/spinner.rs` | `code-rs/core/src/config_types.rs` (`SpinnerSelection`) |
| Change gradient/reveal visuals on cards | `code-rs/tui/src/gradient_background.rs` | `code-rs/tui/src/card_theme.rs`, `code-rs/tui/src/glitch_animation.rs` |
| Change Auto Drive card visual language | `code-rs/tui/src/chatwidget/auto_drive_cards.rs` | `code-rs/tui/src/auto_drive_style.rs`, `code-rs/tui/src/bottom_pane/auto_coordinator_view.rs` |
| Change onboarding screens and first-run visuals | `code-rs/tui/src/onboarding/` | `code-rs/tui/src/app/init.rs` |
| Change alternate-screen behavior (fullscreen vs inline terminal) | `code-rs/tui/src/tui.rs` | `docs/tui-alternate-screen.md`, `code-rs/tui/src/insert_history.rs` |
| Change standard-terminal scrollback insertion behavior | `code-rs/tui/src/insert_history.rs` | `code-rs/tui/src/chatwidget.rs` (`standard_terminal_mode` branches) |

Quick rule of thumb for UI edits:
- If it changes **layout/composition**, start in `chatwidget.rs` + `layout_scroll.rs`.
- If it changes **input/footer/settings panes**, start in `bottom_pane/`.
- If it changes **message visuals**, start in `history_cell/` + markdown renderers.
- If it changes **color/motion style**, start in `theme.rs`, `colors.rs`, and animation modules.

## Tests and Fixtures (Primary)

| Path | Notes | Tags |
|---|---|---|
| `code-rs/core/tests/` | Core behavioral/integration tests | `tests` |
| `code-rs/tui/tests/` | TUI regression + snapshot tests (including VT100 harness) | `tests`, `ui` |
| `code-rs/cloud-tasks/tests/` | Cloud task flow tests | `tests` |
| `code-rs/mcp-types/tests/` | MCP type compatibility tests | `tests`, `protocol` |
| `code-rs/execpolicy/tests/` | Policy parsing/execution checks | `tests` |
| `code-rs/apply-patch/tests/` | Patch parsing/apply behavior coverage | `tests` |

## Non-Rust Components

| Path | Responsibility | Tags |
|---|---|---|
| `codex-cli/bin/coder.js` | Node launcher/wrapper for packaged platform binaries | `entrypoint` |
| `codex-cli/postinstall.js` | Binary install/bootstrap for npm distribution | `tooling` |
| `sdk/typescript/src/` | TS SDK runtime (`codex.ts`, events, thread, exec wrappers) | `integration` |
| `shell-tool-mcp/src/index.ts` | Shell MCP server implementation entrypoint | `integration`, `entrypoint` |

## Build and Release Flow (Where to look)

| Path | Purpose | Tags |
|---|---|---|
| `build-fast.sh` | Required local completion check in this repo | `tooling` |
| `pre-release.sh` | Local release preflight wrapper | `tooling` |
| `.github/workflows/rust-ci.yml` | Rust CI jobs | `tooling` |
| `.github/workflows/release.yml` | Release publication pipeline | `tooling` |
| `scripts/wait-for-gh-run.sh` | Poll/wait helper for GH Actions runs | `tooling` |

## Common Change Routes

| If you need to... | Start here | Then check |
|---|---|---|
| Add/modify a slash command | `code-rs/core/src/slash_commands.rs` | `code-rs/tui/src/slash_command.rs`, `code-rs/protocol/src/parse_command.rs` |
| Change command execution rendering | `code-rs/core/src/codex/exec.rs` | `code-rs/tui/src/chatwidget.rs`, `code-rs/tui/src/history_cell/exec.rs` |
| Update config behavior | `code-rs/core/src/config.rs` | `code-rs/core/src/config/defaults.rs`, `code-rs/core/src/config/validation.rs` |
| Adjust Auto Drive behavior | `code-rs/code-auto-drive-core/src/auto_coordinator.rs` | `code-rs/tui/src/chatwidget/auto_drive_cards.rs`, `code-rs/tui/src/bottom_pane/auto_coordinator_view.rs` |
| Modify browser tools | `code-rs/browser/src/tools/browser_tools.rs` | `code-rs/tui/src/chatwidget/browser_sessions.rs`, tool schema in `code-rs/browser/src/tools/schema.rs` |
| Change MCP tool/server behavior | `code-rs/mcp-server/src/tool_handlers/` | `code-rs/mcp-client/src/`, `code-rs/protocol/src/mcp_protocol.rs` |
| Tune file fuzzy search | `code-rs/file-search/src/lib.rs` | `code-rs/tui/src/file_search.rs` |
| Adjust login/auth UX | `code-rs/login/src/` | `code-rs/core/src/auth.rs`, `code-rs/tui/src/onboarding/` |

## Practical Notes for Agents

- Prefer editing files under `code-rs/` for Rust changes in this fork.
- Treat `codex-rs/` as a reference mirror only unless a task explicitly asks for
  mirror-sync work.
- `AGENTS.md`, `agents.md`, and `CLAUDE.md` are linked instruction surfaces in
  this repo; update instructions consistently.
- When adding new crates, major directories, or ownership boundaries, update
  this file in the same PR so navigation guidance stays current.
