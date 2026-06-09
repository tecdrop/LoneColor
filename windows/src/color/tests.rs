// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! Unit tests for color parsing, formatting, and conversion.

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
