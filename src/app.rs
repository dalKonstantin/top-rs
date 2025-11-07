use crate::proc::mem_info::MemInfo;
use std::time::Instant;

//#[derive(Default)]
pub struct App {
    mem_info: MemInfo,
    last_upd: Instant,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            mem_info: MemInfo::new(),
            last_upd: Instant::now(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.last_upd.elapsed().as_secs() >= 1 {
                self.mem_info.parse();
                self.last_upd = Instant::now();
                ctx.request_repaint();
            }
        });

        crate::ui::draw_main_ui(ctx, &self.mem_info);
    }
}
