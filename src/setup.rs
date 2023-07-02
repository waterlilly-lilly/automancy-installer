use chrono::Local;
use eframe::egui::{Color32, FontData, FontDefinitions, Rounding, Visuals};
use eframe::egui::FontFamily::{Monospace, Proportional};
use crate::InstallerApp;

impl InstallerApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals {
            panel_fill: Color32::WHITE,
            menu_rounding: Rounding::same(5.0),
            ..Default::default()
        });
        let mut fonts = FontDefinitions::default();
        let iosevka = "iosevka";

        fonts
            .font_data
            .insert(iosevka.to_owned(), FontData::from_static(include_bytes!("../include/iosevka-extended.ttf")));

        fonts
            .families
            .get_mut(&Proportional)
            .unwrap()
            .insert(0, iosevka.to_owned());
        fonts
            .families
            .get_mut(&Monospace)
            .unwrap()
            .insert(0, iosevka.to_owned());

        cc.egui_ctx.set_fonts(fonts);
        Self {
            start_time: Local::now(),
            ..Default::default()
        }
    }
}