use egui::{TextStyle, Ui};
use egui_extras::{Column, TableBuilder};

use crate::proc::ProcessInfo;

pub fn show(ui: &mut Ui, processes: &[ProcessInfo]) {
    ui.heading("Processes");

    TableBuilder::new(ui)
        .striped(true) // чередование фона строк
        .column(Column::exact(80.0)) // PID
        .column(Column::remainder()) // Command
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("PID");
            });
            header.col(|ui| {
                ui.strong("Command");
            });
        })
        .body(|body| {
            body.rows(20.0, processes.len(), |mut row| {
                let proc = &processes[row.index()];
                row.col(|ui| {
                    ui.label(proc.pid.to_string());
                });
                row.col(|ui| {
                    ui.label(&proc.command);
                });
            });
        });
}
