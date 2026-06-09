// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! End-to-end tests that launch the app the way users do - a renamed executable,
//! or a shortcut whose name carries the parameters - and assert the resulting
//! wallpaper color and clipboard text. The matrix runs from both start states:
//! an image wallpaper (the common case) and a solid color.
//!
//! Ignored by default: they need an interactive desktop and they change (then
//! restore) the real wallpaper. The image-start tests also require an image
//! wallpaper to be set, and fail with a precondition message if it is not. Run:
//!
//! ```text
//! cargo test --test launch -- --ignored
//! ```

mod common;

use common::{Case, Expect, Harness, Launch, StartState};

const FORMATS: &[Case] = &[
    Case { args: "red", seed_clipboard: None, expect: Expect::Hex("#FF0000") },
    Case { args: "rebeccapurple", seed_clipboard: None, expect: Expect::Hex("#663399") },
    Case { args: "#52A521", seed_clipboard: None, expect: Expect::Hex("#52A521") },
    Case { args: "52A521", seed_clipboard: None, expect: Expect::Hex("#52A521") },
    Case { args: "#abc", seed_clipboard: None, expect: Expect::Hex("#AABBCC") },
    Case { args: "rgb(82,165,33)", seed_clipboard: None, expect: Expect::Hex("#52A521") },
    // R != G != B - catches a wrong COLORREF byte order through the real API.
    Case { args: "#123456", seed_clipboard: None, expect: Expect::Hex("#123456") },
    Case { args: "#000000", seed_clipboard: None, expect: Expect::Hex("#000000") },
    Case { args: "#FFFFFF", seed_clipboard: None, expect: Expect::Hex("#FFFFFF") },
];

const SWITCHES: &[Case] = &[
    // A switch must not clobber the color, in either order (regression for the old C# bug).
    Case { args: "s red", seed_clipboard: None, expect: Expect::Hex("#FF0000") },
    Case { args: "red s", seed_clipboard: None, expect: Expect::Hex("#FF0000") },
];

const SOURCES: &[Case] = &[
    // A color on the clipboard is used by a bare run.
    Case { args: "", seed_clipboard: Some("#52A521"), expect: Expect::Hex("#52A521") },
    // Non-color clipboard on a bare run falls through to a random color.
    Case { args: "", seed_clipboard: None, expect: Expect::Random },
];

const ERRORS: &[Case] = &[
    Case {
        args: "wat",
        seed_clipboard: None,
        expect: Expect::Error("LoneColor Error: Invalid parameter: wat"),
    },
    Case {
        args: "red blue",
        seed_clipboard: None,
        expect: Expect::Error(
            "LoneColor Error: The wallpaper color has been specified twice: \"red\" and \"blue\"",
        ),
    },
];

fn all_cases() -> impl Iterator<Item = &'static Case> {
    FORMATS.iter().chain(SWITCHES).chain(SOURCES).chain(ERRORS)
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn exe_matrix_from_image() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    for case in all_cases() {
        h.run(case, Launch::Exe, StartState::Image);
    }
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop; changes the real wallpaper"]
fn exe_matrix_from_color() {
    let h = Harness::bootstrap();
    for case in all_cases() {
        h.run(case, Launch::Exe, StartState::Color);
    }
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn shortcut_smoke_from_image() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    h.run(&FORMATS[0], Launch::Shortcut, StartState::Image); // a named color, via shortcut
    h.run(&ERRORS[0], Launch::Shortcut, StartState::Image); // the error path, via shortcut
}
