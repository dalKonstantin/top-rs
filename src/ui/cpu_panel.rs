use crate::proc::CpuInfo;
use egui::Ui;
pub fn show(ui: &mut Ui, cpu: &CpuInfo) {
    ui.heading("CPU");
    ui.add(egui::Label::new(cpu.name.clone()));
}
