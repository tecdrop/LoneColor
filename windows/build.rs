// Copyright 2012-2026 Tecdrop
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://www.tecdrop.com/lonecolor/license/.

//! Embeds the app icon and Windows version info into the executable.

fn main() {
    // build.rs runs on the host, so detect the *target* OS via the env var.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/LoneColorIcon.ico");
        res.set("ProductName", "LoneColor");
        res.set("FileDescription", "LoneColor");
        res.set("LegalCopyright", "Copyright (c) 2012-2026 Tecdrop");
        if let Err(error) = res.compile() {
            println!("cargo:warning=winresource failed to embed resources: {error}");
        }
    }
}
