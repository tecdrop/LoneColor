// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! End-to-end tests that launch the app the way users do and assert the resulting
//! wallpaper color and clipboard text. Every color format runs through all three
//! input paths - a renamed executable, a renamed shortcut, and the clipboard - and
//! from both start states, an image wallpaper and a solid color.
//!
//! Each step prints what it is testing and holds the set color on screen briefly,
//! so the run can be watched. Pass `--nocapture` to see the progress messages:
//!
//! ```text
//! cargo test --test launch -- --ignored --nocapture --test-threads=1
//! ```
//!
//! Ignored by default: they need an interactive desktop with an image wallpaper set
//! (they fail with a precondition message otherwise), and they change - then restore
//! - the real wallpaper.

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

fn start_label(start: StartState) -> &'static str {
    match start {
        StartState::Image => "image to color",
        StartState::Color => "color to color",
    }
}

/// Runs the format matrix through one input path, from both start states, narrating each step.
fn format_section(h: &Harness, input: Input, extras: &[ColorCase]) {
    for start in [StartState::Image, StartState::Color] {
        println!("\n  -- {} --", start_label(start));
        for case in FORMATS.iter().chain(extras) {
            println!("    Testing {}", case.text);
            h.run_color(case, input, start);
        }
    }
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn exe_name_input_path() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    println!("\n=== Input path: executable name ===");
    format_section(&h, Input::ExeName, &[]);
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn shortcut_name_input_path() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    println!("\n=== Input path: shortcut name ===");
    format_section(&h, Input::ShortcutName, &[]);
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn clipboard_input_path() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    println!("\n=== Input path: clipboard ===");
    format_section(&h, Input::Clipboard, CLIPBOARD_EXTRA);
}

#[test]
#[serial_test::file_serial]
#[ignore = "needs an interactive desktop with an image wallpaper; changes the real wallpaper"]
fn name_parsing_scenarios() {
    let h = Harness::bootstrap();
    h.require_image_baseline();
    println!("\n=== Name-parsing scenarios (executable name) ===");
    for start in [StartState::Image, StartState::Color] {
        println!("\n  -- {} --", start_label(start));
        for case in SWITCHES.iter().chain(ERRORS) {
            println!("    Testing {}", case.args);
            h.run(case, Launch::Exe, start);
        }
        println!("    Testing (random)");
        h.run(&RANDOM, Launch::Exe, start);
    }
}
