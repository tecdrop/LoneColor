// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

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
