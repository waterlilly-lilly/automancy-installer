use std::process::exit;
use eframe::egui;
use eframe::egui::{Layout, Align, Ui};
use crate::stages::InstallerStage;

pub fn welcome(ui: &mut Ui, stage: &mut InstallerStage) {
    ui.with_layout(Layout::left_to_right(Align::LEFT).with_main_wrap(true), |ui| {
        ui.label("\tWelcome to the automancy installer. This will install automancy version 0.1.0 on \
        your computer. \n\tPress the Start Installation button to continue with the installation, or press \
        Cancel to close the installer.");
    });
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.add_space(150.0);
        if ui.button("Cancel").clicked() {
            *stage = InstallerStage::ExitConfirmation;
        }
        if ui.button("Start Installation").clicked() {
            *stage = InstallerStage::PathSelect;
        }
    });

}