//! Shared popup-related constants for bottom pane widgets.

use crossterm::event::KeyCode;
use ratatui::text::Line;

use crate::key_hint;

/// Maximum number of rows any popup should attempt to display.
/// Keep this consistent across all popups for a uniform feel.
pub(crate) const MAX_POPUP_ROWS: usize = 8;

/// Standard footer hint text used by popups.
pub(crate) fn standard_popup_hint_line() -> Line<'static> {
    let (press_label, confirm_label, back_label) = if crate::i18n::is_chinese() {
        ("按 ", " 确认，或 ", " 返回")
    } else {
        ("Press ", " to confirm or ", " to go back")
    };
    Line::from(vec![
        press_label.into(),
        key_hint::plain(KeyCode::Enter).into(),
        confirm_label.into(),
        key_hint::plain(KeyCode::Esc).into(),
        back_label.into(),
    ])
}
