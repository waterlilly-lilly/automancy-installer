use std::process::exit;
use eframe::egui::Ui;
use crate::stages::InstallerStage;

pub fn exit_confirmation(ui: &mut Ui, stage: &mut InstallerStage, prev_state: Option<InstallerStage>) {
    ui.label("If you exit now, the installation will be canceled. Changes made to the filesystem will NOT be undone. Are you sure?");
    ui.horizontal(|ui| {
        if ui.button("Cancel Installation").clicked() {
            exit(0);
        }
        if ui.button("Continue Installation").clicked() {
            *stage = prev_state.unwrap_or(InstallerStage::Welcome);
        }
    });
}