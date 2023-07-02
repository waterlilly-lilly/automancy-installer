mod setup;
mod stages;

use std::error::Error;
use std::time::Duration;
use chrono::{DateTime, Local};
use eframe::egui;
use eframe::egui::{Frame, Rounding, Style, Visuals, ecolor, Color32, Window, CentralPanel, FontData, FontDefinitions, RichText, vec2, ProgressBar, Widget, Ui, TextStyle};
use eframe::egui::FontFamily::{Monospace, Proportional};
use eframe::epaint::Shadow;
use crate::stages::InstallerStage;

fn main() -> Result<(), eframe::Error>{
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(400.0, 180.0)),
        ..Default::default()
    };
    eframe::run_native(
        "automancy installer",
        options,
        Box::new(|cc| Box::new(InstallerApp::new(cc)))
    )
}
#[derive(Default)]
struct InstallerApp {
    start_time: DateTime<Local>,
    installer_stage: InstallerStage,
    prev_stage: Option<InstallerStage>,
    install_path: Option<String>
}
impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                match self.installer_stage {
                    InstallerStage::UpdateCheck => stages::update_check::update_check(ui, &mut self.installer_stage, &mut self.start_time),
                    InstallerStage::Welcome => stages::welcome::welcome(ui, &mut self.installer_stage),
                    InstallerStage::ExitConfirmation => stages::exit_confirmation::exit_confirmation(ui, &mut self.installer_stage, self.prev_stage),
                    _ => {ui.label("invalid state. please restart installer");}
                }
            });
        });
    }
    fn on_close_event(&mut self) -> bool {
        self.prev_stage = Some(self.installer_stage);
        self.installer_stage = InstallerStage::ExitConfirmation;
        return false;
    }
}
