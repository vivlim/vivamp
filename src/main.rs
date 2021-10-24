#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::path::PathBuf;

use structopt::StructOpt;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let opt = Opt::from_args();

    let mut app = eframe_template::TemplateApp::default();
    if let Some(skin) = opt.skin {
        app.skin_path = Some(skin);
    }
    let mut native_options = eframe::NativeOptions::default();
    //native_options.decorated = false;
    eframe::run_native(Box::new(app), native_options);
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    pub skin: Option<PathBuf>
}