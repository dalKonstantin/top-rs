// ui/mod.rs
use crate::proc::{CpuInfo, MemInfo};
use egui::Context;

pub mod cpu_panel;
pub mod memory_panel;
pub fn draw_main_ui(ctx: &Context, mem: &MemInfo, cpu: &CpuInfo) {
    egui::CentralPanel::default().show(ctx, |ui| {
        cpu_panel::show(ui, cpu);
        ui.separator();
        memory_panel::show(ui, mem);
        ui.separator();
    });
}
