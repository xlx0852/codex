//! Shared popup-related constants for bottom pane widgets.

use crossterm::event::KeyCode;
use ratatui::text::Line;

use crate::key_hint;

/// Maximum number of rows any popup should attempt to display.
/// Keep this consistent across all popups for a uniform feel.
pub(crate) const MAX_POPUP_ROWS: usize = 8;

/// Standard footer hint text used by popups.
pub(crate) fn standard_popup_hint_line() -> Line<'static> {
    Line::from(vec![
        crate::i18n::t!("popup_press").into(),
        key_hint::plain(KeyCode::Enter).into(),
        crate::i18n::t!("popup_to_confirm").into(),
        key_hint::plain(KeyCode::Esc).into(),
        crate::i18n::t!("popup_to_go_back").into(),
    ])
}
