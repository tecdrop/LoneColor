//! The application error type.

use thiserror::Error;

/// Any failure during a run; its `Display` text is what gets copied to the clipboard.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("The wallpaper color has been specified twice: \"{0}\" and \"{1}\"")]
    DuplicateColor(String, String),

    #[error("Clipboard error: {0}")]
    Clipboard(String),

    #[error("Failed to open the control panel: {0}")]
    ControlPanel(#[from] std::io::Error),

    #[error(transparent)]
    Windows(#[from] windows::core::Error),
}
