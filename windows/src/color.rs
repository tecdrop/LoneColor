// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! Color parsing, formatting, and random generation.

use windows::Win32::Foundation::COLORREF;
use windows::Win32::Security::Cryptography::ProcessPrng;

/// An opaque RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Parses a CSS color string: names, hex (with or without `#`), `rgb()`, `hsl()`, `hwb()`.
    pub fn parse(text: &str) -> Option<Self> {
        let [r, g, b, _] = csscolorparser::parse(text.trim()).ok()?.to_rgba8();
        Some(Self { r, g, b })
    }

    /// Returns a random opaque color from the OS cryptographic RNG.
    pub fn random() -> Self {
        let mut bytes = [0u8; 3];
        let _ = unsafe { ProcessPrng(&mut bytes) };
        Self {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
        }
    }

    /// Formats as an uppercase `#RRGGBB` string.
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Converts to a Win32 `COLORREF` (`0x00BBGGRR`).
    pub fn to_colorref(self) -> COLORREF {
        COLORREF(self.r as u32 | (self.g as u32) << 8 | (self.b as u32) << 16)
    }
}

#[cfg(test)]
mod tests;
