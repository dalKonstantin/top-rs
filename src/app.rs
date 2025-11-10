use crate::proc::{CpuInfo, MemInfo};
use std::time::{Duration, Instant};

pub struct App {
    mem_info: MemInfo,
    cpu_info: CpuInfo,
    last_upd: Instant,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            mem_info: MemInfo::new(),
            cpu_info: CpuInfo::new(),
            last_upd: Instant::now(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if now.duration_since(self.last_upd).as_millis() >= 1000 {
            self.mem_info.parse();
            self.cpu_info.parse();
            self.last_upd = now;
        }

        use crate::ui;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::cpu_panel::show(ui, &self.cpu_info);
            ui::memory_panel::show(ui, &self.mem_info);
        });

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
