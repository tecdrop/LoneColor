//! Sets the desktop background to a solid color via the IDesktopWallpaper COM interface.

use windows::Win32::System::Com::{
    CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
};
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};

use crate::color::Rgb;
use crate::error::AppError;

/// Fills the desktop with the given solid color.
///
/// Sets the background color, then disables the wallpaper image so the color shows.
pub fn set_color(color: Rgb) -> Result<(), AppError> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        let wallpaper: IDesktopWallpaper =
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER)?;
        wallpaper.SetBackgroundColor(color.to_colorref())?;
        wallpaper.Enable(false)?;
    }
    Ok(())
}
