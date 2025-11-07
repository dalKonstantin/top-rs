// ui/mod.rs
use crate::proc::mem_info::MemInfo;
use egui::Context; // ← нужно для Context // ← нужно для MemInfo

pub mod memory_panel; // ← объявляем подмодуль

pub fn draw_main_ui(ctx: &Context, mem: &MemInfo) {
    egui::CentralPanel::default().show(ctx, |ui| {
        memory_panel::show(ui, mem); // ← вызов функции из подмодуля
        ui.separator();
    });
}
