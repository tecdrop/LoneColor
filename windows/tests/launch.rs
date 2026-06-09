// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! End-to-end tests that launch the app the way users do and assert the resulting
//! wallpaper color and clipboard text. Every color format is exercised through all
//! three input paths - a renamed executable, a renamed shortcut, and the clipboard.
//!
//! Ignored by default: they need an interactive desktop and they change (then
//! restore) the real wallpaper. The image-start tests also require an image
//! wallpaper to be set, and fail with a precondition message if it is not. Run:
//!
//! ```text
//! cargo test --test launch -- --ignored
//! ```

mod common;

use common::{Case, ColorCase, Expect, Harness, Input, Launch, StartState};

/// The color formats, each with the `#RRGGBB` it resolves to. Run through every input path.
const FORMATS: &[ColorCase] = &[
    ColorCase { text: "red", expect: "#FF0000" },
    ColorCase { text: "rebeccapurple", expect: "#663399" },
    ColorCase { text: "#52A521", expect: "#52A521" },
    ColorCase { text: "52A521", expect: "#52A521" },
    ColorCase { text: "#abc", expect: "#AABBCC" },
    ColorCase { text: "rgb(82,165,33)", expect: "#52A521" },
    // R != G != B - catches a wrong COLORREF byte order through the real API.
    ColorCase { text: "#123456", expect: "#123456" },
    ColorCase { text: "#000000", expect: "#000000" },
    ColorCase { text: "#FFFFFF", expect: "#FFFFFF" },
];

/// Forms only the clipboard accepts - the name path splits on spaces.
const CLIPBOARD_EXTRA: &[ColorCase] = &[
    ColorCase { text: "rgb(82, 165, 33)", expect: "#52A521" }, // spaces inside the function
    ColorCase { text: "  #52A521  ", expect: "#52A521" },      // surrounding whitespace, trimmed
];

const SWITCHES: &[Case] = &[
    // A switch must not clobber the color, in either order (regression for the old C# bug).
    Case { args: "s red", seed_clipboard: None, expect: Expect::Hex("#FF0000") },
    Case { args: "red s", seed_clipboard: None, expect: Expect::Hex("#FF0000") },
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

const RANDOM: Case = Case { args: "", seed_clipboard: None, expect: Expect::Random };

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn exe_format_matrix() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    for case in FORMATS {
        h.run_color(case, Input::ExeName, StartState::Image);
    }
    for case in FORMATS {
        h.run_color(case, Input::ExeName, StartState::Color);
    }
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop; changes the real wallpaper"]
fn shortcut_format_matrix() {
    let h = Harness::bootstrap();
    for case in FORMATS {
        h.run_color(case, Input::ShortcutName, StartState::Color);
    }
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop; changes the real wallpaper"]
fn clipboard_format_matrix() {
    let h = Harness::bootstrap();
    for case in FORMATS.iter().chain(CLIPBOARD_EXTRA) {
        h.run_color(case, Input::Clipboard, StartState::Color);
    }
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn name_scenarios_from_image() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    for case in SWITCHES.iter().chain(ERRORS) {
        h.run(case, Launch::Exe, StartState::Image);
    }
    h.run(&RANDOM, Launch::Exe, StartState::Image);
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn shortcut_smoke_from_image() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    let red = Case { args: "red", seed_clipboard: None, expect: Expect::Hex("#FF0000") };
    h.run(&red, Launch::Shortcut, StartState::Image); // a named color, via shortcut
    h.run(&ERRORS[0], Launch::Shortcut, StartState::Image); // the error path, via shortcut
}
