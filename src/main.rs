#![cfg(target_os = "linux")]
mod app;
mod proc;
mod ui;
use tracing::{debug, info};

use crate::app::App;
fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting rtop!");
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "rtop",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
