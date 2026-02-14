## 1. Implementation
- [x] 1.1 Localize footer shortcut labels and transient footer hints in `footer.rs`.
- [x] 1.2 Localize collaboration mode labels and context window text in `footer.rs`.
- [x] 1.3 Localize slash popup command descriptions in `slash_command.rs`.
- [x] 1.4 Localize startup tooltip copy and session header model-change hint text.
- [x] 1.5 Add Chinese/English footer text mapping for translated footer copy in `footer.rs`.
- [x] 1.6 Localize secondary selection menu titles, subtitles, and search placeholders in `chatwidget.rs`.
- [x] 1.7 Localize shared popup hint copy in `popup_consts.rs`.
- [x] 1.8 Localize skills menu copy in `chatwidget/skills.rs`.
- [x] 1.9 Localize empty-state copy (`no matches`) in selection popups.
- [x] 1.10 Fix locale YAML structure so i18n keys resolve to translated content.
- [x] 1.11 Localize onboarding trust-directory key prompts in `trust_directory.rs`.
- [x] 1.12 Localize running status-line hint copy (`Working`, `esc to interrupt`).
- [x] 1.13 Localize random composer placeholder suggestions.
- [x] 1.14 Run formatter and crate tests for `codex-tui`.
- [x] 1.15 Localize agent picker and resume/fork session picker copy (title/subtitle/search/empty-state/hints).
- [x] 1.16 Localize feedback, request-user-input, unified-exec, app-link, and experimental-features popup copy.
- [x] 1.17 Localize onboarding auth/device-code, OSS selector, model-migration prompt copy, and key status/history empty states.

## 2. Validation
- [x] 2.1 Confirm existing footer snapshot coverage still passes under default locale.
- [x] 2.2 Verify Chinese locale path produces translated footer labels in code-level assertions.
- [x] 2.3 Confirm trust-directory snapshot now renders translated values instead of unresolved key names.
- [x] 2.4 Confirm `codex-tui` test suite passes after status-line/placeholder locale updates.
- [x] 2.5 Accept intentional snapshot deltas introduced by updated localized copy and re-run `cargo test -p codex-tui`.
