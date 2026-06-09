// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! LoneColor for Windows - fills the desktop with a solid color, then exits.
//!
//! The color comes from the launch name, the clipboard, or a random fallback.

#![windows_subsystem = "windows"]

mod clipboard;
mod color;
mod control_panel;
mod error;
mod params;
mod startup;
mod wallpaper;

use error::AppError;
use params::Params;

fn main() {
    let mut params = Params::default();
    if let Err(error) = run(&mut params) {
        if !params.silent {
            error_sound();
        }
        let _ = clipboard::set(&format!("LoneColor Error: {error}"));
    }
}

/// Resolves the color (name, then clipboard, then random) and applies it.
fn run(params: &mut Params) -> Result<(), AppError> {
    params.parse(&startup::launch_name())?;

    if params.choose {
        return control_panel::open_desktop_background();
    }

    let color = params
        .color
        .or_else(clipboard::get_color)
        .unwrap_or_else(color::Rgb::random);

    clipboard::set(&format!("Color {}", color.to_hex()))?;
    wallpaper::set_color(color)
}

/// Plays the system error sound.
fn error_sound() {
    use windows::Win32::System::Diagnostics::Debug::MessageBeep;
    use windows::Win32::UI::WindowsAndMessaging::MB_ICONHAND;
    unsafe {
        let _ = MessageBeep(MB_ICONHAND);
    }
}
