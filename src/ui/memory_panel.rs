use crate::proc::MemInfo;
use egui::Ui;
pub fn show(ui: &mut Ui, mem: &MemInfo) {
    ui.heading("Memory");
    ui.add(
        egui::ProgressBar::new(mem.used_percent() / 100.0)
            .text(format!("{:.1}%", mem.used_percent())),
    );
}
