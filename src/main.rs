mod app;
mod proc;
use proc::mem_info::MemInfo;
mod ui;
use ui::memory_panel;

use crate::app::App;
fn main() {
    let m = MemInfo::new();
    println!("{:#?}", m);
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "rtop",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
