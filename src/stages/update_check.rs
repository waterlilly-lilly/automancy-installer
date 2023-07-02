use std::thread::sleep;
use chrono::{DateTime, Duration, Local};
use eframe::egui::{Color32, Spinner, Ui};
use crate::stages::InstallerStage;

pub fn update_check(ui: &mut Ui, stage: &mut InstallerStage, timeout: &mut DateTime<Local>) {
    ui.label("Checking for updates, please wait...");
    ui.label("(this doesn't actually do anything rn)");
    ui.add(Spinner::new().size(32.0).color(Color32::LIGHT_BLUE));
    if Local::now().signed_duration_since(timeout.clone()) > Duration::seconds(3) {
        *stage = InstallerStage::Welcome;
    }
}