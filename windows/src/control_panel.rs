//! Opens the Desktop Background settings - the "choose" / undo path.

use std::process::Command;

use crate::error::AppError;

/// Opens the Personalization desktop-background page so the user can restore a wallpaper.
pub fn open_desktop_background() -> Result<(), AppError> {
    Command::new("control.exe")
        .args([
            "/name",
            "Microsoft.Personalization",
            "/page",
            "pageWallpaper",
        ])
        .spawn()?;
    Ok(())
}
