// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! Shared harness for the launch-mechanism end-to-end tests.
//!
//! Bootstraps one copy of the built binary and one shortcut, renames them per
//! case (the launch name carries the parameters), runs them, and asserts the
//! resulting wallpaper and clipboard state through the live IDesktopWallpaper API.
//!
//! Each case starts from a chosen wallpaper state - the user's real image (the
//! common scenario) or a solid baseline color - which is applied and settled
//! before launch, so the shell's asynchronous wallpaper updates cannot race the
//! read-back. The prior wallpaper and clipboard are snapshotted once and restored
//! when the harness drops.

#![allow(dead_code)]

use std::ffi::{OsStr, c_void};
use std::io::Write;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use windows::Win32::Foundation::{CloseHandle, COLORREF};
use windows::Win32::System::Com::{
    CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
};
use windows::Win32::System::Threading::WaitForSingleObject;
use windows::Win32::UI::Shell::{
    DesktopWallpaper, IDesktopWallpaper, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW,
    ShellExecuteExW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{PCWSTR, PWSTR};

/// Non-color clipboard text used when a case seeds nothing, so a bare run takes
/// the random path instead of an accidental leftover color.
const SENTINEL: &str = "lonecolor-e2e-sentinel";

/// The baseline for color-start cases - a distinct color unlikely to equal a test color.
const BASELINE: COLORREF = COLORREF(0x0001_0203);

/// How long to let an image apply settle before launching. `SetWallpaper` finishes
/// asynchronously and exposes no completion signal, and a late finish re-shows the
/// image after the run has disabled it. This wait covers that tail; it is the knob
/// to raise if a slow machine flakes.
const IMAGE_APPLY_WAIT: Duration = Duration::from_millis(1500);

/// How long to keep a set wallpaper on screen before moving on, so the change is
/// clearly visible to someone watching the run.
const VISIBLE_HOLD: Duration = Duration::from_millis(1000);

/// How the app is launched - the two real parameter-bearing mechanisms.
#[derive(Clone, Copy)]
pub enum Launch {
    Exe,
    Shortcut,
}

/// The wallpaper state a case starts from.
#[derive(Clone, Copy)]
pub enum StartState {
    /// The user's current image wallpaper - the common real scenario.
    Image,
    /// A solid baseline color.
    Color,
}

/// The expected outcome of a run.
pub enum Expect {
    /// Deterministic color: clipboard `Color <hex>`, wallpaper equals `<hex>`, no image.
    Hex(&'static str),
    /// Random color: clipboard `Color #RRGGBB`, wallpaper equals that color, no image.
    Random,
    /// Error: clipboard equals the text, wallpaper unchanged from the start state.
    Error(&'static str),
}

/// One end-to-end case: the launch-name parameters and what should result.
pub struct Case {
    /// The text after `LoneColor` in the launch name; empty means a bare run.
    pub args: &'static str,
    /// Clipboard text to seed before the run (for the clipboard-sourced color path).
    pub seed_clipboard: Option<&'static str>,
    pub expect: Expect,
}

/// The path a color text is delivered through - the three ways a user gives LoneColor a color.
#[derive(Clone, Copy, Debug)]
pub enum Input {
    /// In the executable's name.
    ExeName,
    /// In a shortcut's name.
    ShortcutName,
    /// On the clipboard, with a bare-named run.
    Clipboard,
}

/// A color text and the `#RRGGBB` it should resolve to, run through any `Input`.
pub struct ColorCase {
    pub text: &'static str,
    pub expect: &'static str,
}

pub struct Harness {
    _dir: tempfile::TempDir,
    dir: PathBuf,
    exe_hold: PathBuf,
    lnk_hold: PathBuf,
    guard: StateGuard,
}

impl Harness {
    /// Copies the built binary once, creates one shortcut to it, and snapshots the
    /// wallpaper and clipboard for restoration at the end. Both get renamed per case.
    pub fn bootstrap() -> Self {
        let exe_src = PathBuf::from(env!("CARGO_BIN_EXE_LoneColor"));
        let dir = tempfile::tempdir().expect("create temp dir");
        let dirp = dir.path().to_path_buf();

        let exe_hold = dirp.join("_hold.exe");
        std::fs::copy(&exe_src, &exe_hold).expect("copy built exe");

        let lnk_hold = dirp.join("_hold.lnk");
        create_shortcut(&lnk_hold, &exe_src);

        Self { _dir: dir, dir: dirp, exe_hold, lnk_hold, guard: StateGuard::snapshot() }
    }

    /// Fails loudly if the current wallpaper is not an image - the image-start
    /// cases need a real image to transition from.
    pub fn require_image_baseline(&self) {
        assert!(
            !self.guard.images.is_empty(),
            "PRECONDITION: the image-start tests need an image desktop wallpaper, but the \
             current wallpaper is a solid color. Set a photo wallpaper and re-run."
        );
    }

    /// Runs one case from the given start state through the given launch mechanism.
    pub fn run(&self, case: &Case, via: Launch, start: StartState) {
        clipboard_win::set_clipboard_string(case.seed_clipboard.unwrap_or(SENTINEL))
            .expect("seed clipboard");

        let wp = desktop_wallpaper();
        self.apply_start_state(&wp, start);
        announce(if case.args.is_empty() { "(random)" } else { case.args });
        let before = fingerprint(&wp);

        let stem = if case.args.is_empty() {
            "LoneColor".to_string()
        } else {
            format!("LoneColor {}", case.args)
        };
        match via {
            Launch::Exe => self.launch_exe(&stem),
            Launch::Shortcut => self.launch_shortcut(&stem),
        }

        let clip = clipboard_win::get_clipboard_string().unwrap_or_default();
        match &case.expect {
            Expect::Hex(hex) => {
                assert_eq!(clip, format!("Color {hex}"), "clipboard for `{}`", case.args);
                let want = hex_to_colorref(hex);
                if !settle(|| background_color(&wp).0 == want.0 && no_image_showing(&wp)) {
                    let (bg, image) = fingerprint(&wp);
                    panic!(
                        "`{}`: wanted bg={:#010X} no-image, got bg={:#010X} image_showing={}",
                        case.args, want.0, bg, image
                    );
                }
            }
            Expect::Random => {
                let hex = clip
                    .strip_prefix("Color ")
                    .unwrap_or_else(|| panic!("unexpected random clipboard: {clip:?}"));
                let want = hex_to_colorref(hex);
                assert!(
                    settle(|| background_color(&wp).0 == want.0 && no_image_showing(&wp)),
                    "random wallpaper did not settle to {hex} (solid, no image)"
                );
            }
            Expect::Error(text) => {
                assert_eq!(clip, *text, "error clipboard for `{}`", case.args);
                assert_eq!(
                    fingerprint(&wp),
                    before,
                    "wallpaper must be unchanged for error `{}`",
                    case.args
                );
            }
        }
        sleep(VISIBLE_HOLD);
    }

    /// Runs one color case, delivering the color text through the given input path.
    pub fn run_color(&self, case: &ColorCase, input: Input, start: StartState) {
        let wp = desktop_wallpaper();
        self.apply_start_state(&wp, start);
        announce(case.text);

        let (stem, seed, launch) = match input {
            Input::ExeName => (format!("LoneColor {}", case.text), SENTINEL, Launch::Exe),
            Input::ShortcutName => (format!("LoneColor {}", case.text), SENTINEL, Launch::Shortcut),
            Input::Clipboard => ("LoneColor".to_string(), case.text, Launch::Exe),
        };
        clipboard_win::set_clipboard_string(seed).expect("seed clipboard");

        match launch {
            Launch::Exe => self.launch_exe(&stem),
            Launch::Shortcut => self.launch_shortcut(&stem),
        }

        let clip = clipboard_win::get_clipboard_string().unwrap_or_default();
        let label = format!("`{}` via {input:?}", case.text);
        assert_eq!(clip, format!("Color {}", case.expect), "clipboard for {label}");
        let want = hex_to_colorref(case.expect);
        if !settle(|| background_color(&wp).0 == want.0 && no_image_showing(&wp)) {
            let (bg, image) = fingerprint(&wp);
            panic!(
                "{label}: wanted bg={:#010X} no-image, got bg={:#010X} image_showing={}",
                want.0, bg, image
            );
        }
        sleep(VISIBLE_HOLD);
    }

    /// Applies the start state and waits until it is actually showing, so no
    /// in-flight wallpaper change races the run that follows.
    fn apply_start_state(&self, wp: &IDesktopWallpaper, start: StartState) {
        unsafe {
            match start {
                StartState::Image => {
                    for (id, path) in &self.guard.images {
                        let _ = wp.SetWallpaper(PCWSTR(id.as_ptr()), PCWSTR(path.as_ptr()));
                    }
                    let _ = wp.Enable(true);
                }
                StartState::Color => {
                    let _ = wp.SetBackgroundColor(BASELINE);
                    let _ = wp.Enable(false);
                }
            }
        }
        let settled = match start {
            StartState::Image => settle(|| !no_image_showing(wp)),
            StartState::Color => settle(|| background_color(wp).0 == BASELINE.0 && no_image_showing(wp)),
        };
        assert!(settled, "the starting wallpaper state did not settle");
        // Keep the start state on screen a moment so the change is visible. For an
        // image this wait also lets the slow apply finish before launch, so it cannot
        // re-show the image after the run disables it.
        match start {
            StartState::Image => sleep(IMAGE_APPLY_WAIT),
            StartState::Color => sleep(VISIBLE_HOLD),
        }
    }

    fn launch_exe(&self, stem: &str) {
        let cased = self.dir.join(format!("{stem}.exe"));
        std::fs::rename(&self.exe_hold, &cased).expect("rename exe to cased name");
        let status = Command::new(&cased).status();
        std::fs::rename(&cased, &self.exe_hold).expect("rename exe back");
        let status = status.expect("spawn renamed exe");
        assert!(status.success(), "exe exited with failure: {status}");
    }

    fn launch_shortcut(&self, stem: &str) {
        let cased = self.dir.join(format!("{stem}.lnk"));
        std::fs::rename(&self.lnk_hold, &cased).expect("rename shortcut to cased name");
        let before = clipboard_win::get_clipboard_string().unwrap_or_default();
        shell_execute_wait(&cased, &before);
        std::fs::rename(&cased, &self.lnk_hold).expect("rename shortcut back");
    }
}

/// Snapshots wallpaper color, per-monitor image, and clipboard, restoring them on drop.
struct StateGuard {
    wp: IDesktopWallpaper,
    color: COLORREF,
    images: Vec<(Vec<u16>, Vec<u16>)>,
    clipboard: Option<String>,
}

impl StateGuard {
    fn snapshot() -> Self {
        let wp = desktop_wallpaper();
        let color = background_color(&wp);
        let mut images = Vec::new();
        unsafe {
            let count = wp.GetMonitorDevicePathCount().expect("monitor count");
            for i in 0..count {
                let id = wp.GetMonitorDevicePathAt(i).expect("monitor id");
                let image = wp.GetWallpaper(PCWSTR(id.0)).expect("GetWallpaper");
                let image_str = if image.is_null() {
                    String::new()
                } else {
                    image.to_string().unwrap_or_default()
                };
                if !image_str.is_empty() {
                    images.push((wide_str(&id.to_string().unwrap_or_default()), wide_str(&image_str)));
                }
                CoTaskMemFree(Some(image.0 as *const c_void));
                CoTaskMemFree(Some(id.0 as *const c_void));
            }
        }
        Self { wp, color, images, clipboard: clipboard_win::get_clipboard_string().ok() }
    }
}

impl Drop for StateGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = self.wp.SetBackgroundColor(self.color);
            if self.images.is_empty() {
                let _ = self.wp.Enable(false);
            } else {
                for (id, path) in &self.images {
                    let _ = self.wp.SetWallpaper(PCWSTR(id.as_ptr()), PCWSTR(path.as_ptr()));
                }
                let _ = self.wp.Enable(true);
            }
        }
        // An image restore applies asynchronously; wait for it so the image is back
        // before the process exits, instead of leaving the last test color showing.
        if !self.images.is_empty() {
            let _ = settle(|| !no_image_showing(&self.wp));
            sleep(IMAGE_APPLY_WAIT);
        }
        if let Some(text) = &self.clipboard {
            let _ = clipboard_win::set_clipboard_string(text);
        }
    }
}

/// Creates a `.lnk` at `lnk` pointing at `target`, via the WScript.Shell helper.
fn create_shortcut(lnk: &Path, target: &Path) {
    let script = format!(
        "$s=(New-Object -ComObject WScript.Shell).CreateShortcut('{}'); $s.TargetPath='{}'; $s.Save()",
        lnk.display(),
        target.display()
    );
    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .status()
        .expect("run powershell to create the shortcut");
    assert!(status.success(), "shortcut creation failed");
}

/// Launches a shortcut through the shell (so `STARTF_TITLEISLINKNAME` is set),
/// then waits for it to finish - by process handle, or by polling the clipboard
/// if the shell returns no waitable handle for the `.lnk`.
fn shell_execute_wait(lnk: &Path, clipboard_before: &str) {
    let file = wide(lnk.as_os_str());
    let verb = wide_str("open");
    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS,
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(file.as_ptr()),
        nShow: SW_SHOWNORMAL.0,
        ..Default::default()
    };
    unsafe {
        ShellExecuteExW(&mut info).expect("ShellExecuteExW on shortcut");
        if !info.hProcess.is_invalid() {
            WaitForSingleObject(info.hProcess, 5000);
            let _ = CloseHandle(info.hProcess);
        } else {
            poll_clipboard_change(clipboard_before);
        }
    }
}

fn poll_clipboard_change(before: &str) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if clipboard_win::get_clipboard_string().unwrap_or_default() != before {
            return;
        }
        sleep(Duration::from_millis(50));
    }
}

/// Retries `check` for a short window, to absorb the shell applying a wallpaper
/// change asynchronously after the app process has already exited.
fn settle(check: impl Fn() -> bool) -> bool {
    let start = Instant::now();
    loop {
        if check() {
            return true;
        }
        if start.elapsed() > Duration::from_secs(3) {
            return false;
        }
        sleep(Duration::from_millis(50));
    }
}

/// Prints which case is about to run and flushes, so the line lands on screen in
/// step with the wallpaper change rather than buffered ahead of it.
fn announce(label: &str) {
    println!("    Testing {label}");
    let _ = std::io::stdout().flush();
}

fn desktop_wallpaper() -> IDesktopWallpaper {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER).expect("create IDesktopWallpaper")
    }
}

fn background_color(wp: &IDesktopWallpaper) -> COLORREF {
    unsafe { wp.GetBackgroundColor().expect("GetBackgroundColor") }
}

/// True when no monitor is displaying a wallpaper image (all solid color).
fn no_image_showing(wp: &IDesktopWallpaper) -> bool {
    unsafe {
        let count = wp.GetMonitorDevicePathCount().expect("monitor count");
        for i in 0..count {
            let id = wp.GetMonitorDevicePathAt(i).expect("monitor id");
            let image = take_pwstr(wp.GetWallpaper(PCWSTR(id.0)).expect("GetWallpaper"));
            CoTaskMemFree(Some(id.0 as *const c_void));
            if !image.is_empty() {
                return false;
            }
        }
        true
    }
}

/// The wallpaper as a comparable fingerprint: (background color, is an image showing).
fn fingerprint(wp: &IDesktopWallpaper) -> (u32, bool) {
    (background_color(wp).0, !no_image_showing(wp))
}

/// Reads a COM-allocated wide string into a `String` and frees it.
unsafe fn take_pwstr(p: PWSTR) -> String {
    if p.is_null() {
        return String::new();
    }
    let s = unsafe { p.to_string() }.unwrap_or_default();
    unsafe { CoTaskMemFree(Some(p.0 as *const c_void)) };
    s
}

fn hex_to_colorref(hex: &str) -> COLORREF {
    let h = hex.trim_start_matches('#');
    let byte = |i: usize| u32::from_str_radix(&h[i..i + 2], 16).expect("hex byte");
    COLORREF(byte(0) | (byte(2) << 8) | (byte(4) << 16))
}

fn wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}

fn wide_str(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
