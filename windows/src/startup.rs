// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! Determines the name the app was launched as (shortcut or executable).

use std::path::Path;

use windows::Win32::System::Threading::{GetStartupInfoW, STARTF_TITLEISLINKNAME, STARTUPINFOW};

/// Returns the launching file's base name - no directory, no extension.
///
/// Started from a shortcut, this is the shortcut's name; otherwise the executable's.
/// The words of this name carry the run parameters.
pub fn launch_name() -> String {
    let path = shortcut_path()
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .map(|p| p.to_string_lossy().into_owned())
        })
        .unwrap_or_default();
    Path::new(&path)
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// The `.lnk` path when the process was started from a shortcut, else `None`.
fn shortcut_path() -> Option<String> {
    unsafe {
        let mut info = STARTUPINFOW::default();
        GetStartupInfoW(&mut info);
        if info.dwFlags.0 & STARTF_TITLEISLINKNAME.0 != 0 && !info.lpTitle.is_null() {
            info.lpTitle.to_string().ok()
        } else {
            None
        }
    }
}
