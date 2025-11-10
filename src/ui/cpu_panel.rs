use crate::proc::CpuInfo;
use egui::Ui;

pub fn show(ui: &mut Ui, cpu: &CpuInfo) {
    ui.heading("CPU");
    ui.label(&cpu.name);
    ui.add(
        egui::ProgressBar::new(cpu.usage_percent / 100.0)
            .text(format!("{:.1}%", cpu.usage_percent)),
    );
}
