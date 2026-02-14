//! Internationalization (i18n) support for the TUI.
//!
//! This module provides localization capabilities using the `rust-i18n` crate.
//! It supports English and Chinese (Simplified) out of the box, with automatic
//! language detection based on system locale.

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

// Note: rust_i18n::i18n! macro is called in lib.rs to generate the _rust_i18n_translate function at crate root

/// Whether to use Chinese locale.
static USE_ZH: AtomicBool = AtomicBool::new(false);

/// Initialize the i18n system with automatic language detection.
///
/// This function detects the system locale and sets the appropriate language.
/// It checks for Chinese language codes (zh, zh-CN, zh-TW, etc.) and falls back
/// to English for all other locales.
pub fn init() {
    let locale = detect_locale();
    set_locale(&locale);
}

/// Force a specific locale.
///
/// Supported locales:
/// - "en" - English
/// - "zh" - Chinese (Simplified)
pub fn set_locale(locale: &str) {
    let is_zh = locale.starts_with("zh");
    USE_ZH.store(is_zh, Ordering::SeqCst);
    rust_i18n::set_locale(locale);
}

/// Get the current locale.
pub fn current_locale() -> String {
    if USE_ZH.load(Ordering::SeqCst) {
        "zh".to_string()
    } else {
        "en".to_string()
    }
}

/// Check if the current locale is Chinese.
pub fn is_chinese() -> bool {
    USE_ZH.load(Ordering::SeqCst)
}

/// Return a localized string literal pair (`zh`, `en`) based on current locale.
pub fn localized<'a>(zh: &'a str, en: &'a str) -> &'a str {
    if is_chinese() { zh } else { en }
}

/// Detect the system locale.
///
/// This function tries multiple methods to detect the system locale:
/// 1. Check the `LANG` environment variable
/// 2. Check the `LC_ALL` environment variable
/// 3. Default to "en" if no locale is detected
fn detect_locale() -> String {
    // Check LANG environment variable
    if let Ok(lang) = std::env::var("LANG") {
        let lang = lang.split('.').next().unwrap_or(&lang);
        if lang.starts_with("zh") {
            return "zh".to_string();
        }
    }

    // Check LC_ALL environment variable
    if let Ok(lc_all) = std::env::var("LC_ALL") {
        let lc_all = lc_all.split('.').next().unwrap_or(&lc_all);
        if lc_all.starts_with("zh") {
            return "zh".to_string();
        }
    }

    // Default to English
    "en".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_detection() {
        // Save original env vars
        let original_lang = std::env::var("LANG").ok();
        let original_lc_all = std::env::var("LC_ALL").ok();

        unsafe {
            // Test English detection
            std::env::set_var("LANG", "en_US.UTF-8");
            assert_eq!(detect_locale(), "en");

            // Test Chinese detection via LANG
            std::env::set_var("LANG", "zh_CN.UTF-8");
            assert_eq!(detect_locale(), "zh");

            // Test Chinese detection via LC_ALL
            std::env::remove_var("LANG");
            std::env::set_var("LC_ALL", "zh_TW.UTF-8");
            assert_eq!(detect_locale(), "zh");

            // Restore original env vars
            match original_lang {
                Some(val) => std::env::set_var("LANG", val),
                None => std::env::remove_var("LANG"),
            }
            match original_lc_all {
                Some(val) => std::env::set_var("LC_ALL", val),
                None => std::env::remove_var("LC_ALL"),
            }
        }
    }

    #[test]
    fn test_set_locale() {
        set_locale("en");
        assert!(!is_chinese());
        assert_eq!(current_locale(), "en");

        set_locale("zh");
        assert!(is_chinese());
        assert_eq!(current_locale(), "zh");

        // Reset to English for other tests
        set_locale("en");
    }
}
