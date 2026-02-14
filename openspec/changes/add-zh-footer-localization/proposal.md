# Change: Localize Key TUI Hints for Chinese Users

## Why
Chinese users running Codex CLI still see English-only footer hints in many common interaction states. This makes keyboard guidance and mode indicators harder to understand.

## What Changes
- Localize bottom-pane footer labels, mode indicators, and context usage text.
- Localize slash command descriptions shown in the `/` popup.
- Localize startup tooltip copy and session header model-change hint text.
- Localize key secondary selection menus (model, reasoning, collaboration, review, skills) and popup hints.
- Localize shared "no matches" empty-state copy in selection-style popups.
- Fix onboarding trust-directory translation key rendering by correcting locale file structure.
- Localize chat composer placeholder suggestions and running status-line hints (`Working`, `esc to interrupt`).
- Localize agent/session picker copy (`agent` selection popup and resume/fork session picker search/empty states/hints).
- Localize additional high-traffic overlays: feedback popups, request-user-input footer/empty states, unified-exec footer summary, app-link popup, and experimental-features popup.
- Localize onboarding auth/device-code prompts, OSS provider selector copy, and model-migration prompt/menu hints.
- Localize status/history empty-state copy for limits/background terminals/MCP tool lists.
- Keep English behavior unchanged when locale is `en`.
- Keep implementation narrowly scoped to TUI presentation layers.

## Impact
- Affected code:
  - `codex-rs/tui/src/bottom_pane/footer.rs`
  - `codex-rs/tui/src/bottom_pane/list_selection_view.rs`
  - `codex-rs/tui/src/bottom_pane/popup_consts.rs`
  - `codex-rs/tui/src/bottom_pane/command_popup.rs`
  - `codex-rs/tui/src/bottom_pane/file_search_popup.rs`
  - `codex-rs/tui/src/bottom_pane/multi_select_picker.rs`
  - `codex-rs/tui/src/bottom_pane/skill_popup.rs`
  - `codex-rs/tui/src/bottom_pane/skills_toggle_view.rs`
  - `codex-rs/tui/src/bottom_pane/feedback_view.rs`
  - `codex-rs/tui/src/bottom_pane/request_user_input/mod.rs`
  - `codex-rs/tui/src/bottom_pane/request_user_input/render.rs`
  - `codex-rs/tui/src/bottom_pane/unified_exec_footer.rs`
  - `codex-rs/tui/src/bottom_pane/app_link_view.rs`
  - `codex-rs/tui/src/bottom_pane/experimental_features_view.rs`
  - `codex-rs/tui/src/chatwidget.rs`
  - `codex-rs/tui/src/chatwidget/skills.rs`
  - `codex-rs/tui/src/app.rs`
  - `codex-rs/tui/src/resume_picker.rs`
  - `codex-rs/tui/src/status_indicator_widget.rs`
  - `codex-rs/tui/src/status/card.rs`
  - `codex-rs/tui/src/onboarding/auth.rs`
  - `codex-rs/tui/src/onboarding/auth/headless_chatgpt_login.rs`
  - `codex-rs/tui/src/onboarding/trust_directory.rs`
  - `codex-rs/tui/src/oss_selection.rs`
  - `codex-rs/tui/src/model_migration.rs`
  - `codex-rs/tui/locales/en.yml`
  - `codex-rs/tui/locales/zh.yml`
  - `codex-rs/tui/src/slash_command.rs`
  - `codex-rs/tui/src/tooltips.rs`
  - `codex-rs/tui/src/history_cell.rs`
