use crate::proc::{
    CpuInfo, MemInfo,
    process_info::{collect_processes, list_pids},
};
use std::time::{Duration, Instant};

pub struct App {
    mem_info: MemInfo,
    cpu_info: CpuInfo,
    pids: Vec<crate::proc::ProcessInfo>,
    last_upd: Instant,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            mem_info: MemInfo::new(),
            cpu_info: CpuInfo::new(),
            pids: Vec::new(),
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
            self.pids = collect_processes();
            self.last_upd = now;
        }

        use crate::ui;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::cpu_panel::show(ui, &self.cpu_info);
            ui::memory_panel::show(ui, &self.mem_info);
            ui::processes_table::show(ui, &self.pids);
        });

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
