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
mod tests {
    use super::Rgb;

    #[test]
    fn parses_hex_with_and_without_hash() {
        let expected = Rgb {
            r: 0xD3,
            g: 0x19,
            b: 0x96,
        };
        assert_eq!(Rgb::parse("#D31996"), Some(expected));
        assert_eq!(Rgb::parse("D31996"), Some(expected));
    }

    #[test]
    fn parses_named_and_functional_colors() {
        assert_eq!(Rgb::parse("red"), Some(Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(
            Rgb::parse("rgb(82,165,33)"),
            Some(Rgb {
                r: 82,
                g: 165,
                b: 33
            })
        );
    }

    #[test]
    fn rejects_non_colors() {
        assert_eq!(Rgb::parse("silent"), None);
        assert_eq!(Rgb::parse("82,165,33"), None); // bare triplet not accepted in v4
    }

    #[test]
    fn formats_hex_uppercase() {
        assert_eq!(
            Rgb {
                r: 0xD3,
                g: 0x19,
                b: 0x96
            }
            .to_hex(),
            "#D31996"
        );
    }

    #[test]
    fn colorref_is_bgr_ordered() {
        // 0x00BBGGRR
        assert_eq!(
            Rgb {
                r: 0x12,
                g: 0x34,
                b: 0x56
            }
            .to_colorref()
            .0,
            0x0056_3412
        );
    }
}
