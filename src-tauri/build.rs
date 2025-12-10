// This attribute should be in main.rs, not build.rs, to avoid runtime initialization issues.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Uncaught TypeError: Cannot read properties of undefined (reading 'core')
// If you encounter an error during the build process related to resources,
// set the BUILD_RES constant to false below.
// Then set it back to true and build again.
const BUILD_RES: bool = true;

#[cfg(windows)]
fn main() {
    if BUILD_RES {
        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set_manifest(include_str!("windows-manifest.xml"));
        res.compile().unwrap();
    } else {
        tauri_build::build()
    }
}

#[cfg(not(windows))]
fn main() {
   tauri_build::build()
}