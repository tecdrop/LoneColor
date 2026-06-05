//! Clipboard text read/write and color extraction.

use crate::color::Rgb;
use crate::error::AppError;

/// Writes text to the clipboard.
pub fn set(text: &str) -> Result<(), AppError> {
    clipboard_win::set_clipboard_string(text).map_err(|e| AppError::Clipboard(e.to_string()))
}

/// Returns a color if the clipboard currently holds text that parses as one.
pub fn get_color() -> Option<Rgb> {
    Rgb::parse(&clipboard_win::get_clipboard_string().ok()?)
}
