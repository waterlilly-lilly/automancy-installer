use chrono::Local;
use eframe::egui::{Color32, FontData, FontDefinitions, FontId, Rounding, Stroke, Style, TextStyle, Visuals};
use eframe::egui::FontFamily::{Monospace, Proportional};
use eframe::egui::style::{Widgets, WidgetVisuals};
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
        cc.egui_ctx.set_style(Style {
            override_text_style: None,
            override_font_id: None,
            text_styles: [
                (TextStyle::Small, FontId::new(9.0, Proportional)),
                (TextStyle::Body, FontId::new(13.0, Proportional)),
                (TextStyle::Button, FontId::new(13.0, Proportional)),
                (TextStyle::Heading, FontId::new(19.0, Proportional)),
                (TextStyle::Monospace, FontId::new(13.0, Monospace)),
            ]
                .into(),
            wrap: None,
            visuals: Visuals {
                widgets: Widgets {
                    noninteractive: WidgetVisuals {
                        weak_bg_fill: Color32::from_gray(248),
                        bg_fill: Color32::from_gray(170),
                        bg_stroke: Stroke::new(1.0, Color32::from_gray(160)), // separators, indentation lines
                        fg_stroke: Stroke::new(1.0, Color32::from_gray(80)),  // normal text color
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
                    inactive: WidgetVisuals {
                        weak_bg_fill: Color32::from_gray(200), // button background
                        bg_fill: Color32::from_gray(200),      // checkbox background
                        bg_stroke: Default::default(),
                        fg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // button text
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
                    hovered: WidgetVisuals {
                        weak_bg_fill: Color32::from_gray(220),
                        bg_fill: Color32::from_gray(190),
                        bg_stroke: Stroke::new(1.0, Color32::from_gray(105)), // e.g. hover over window edge or button
                        fg_stroke: Stroke::new(1.5, Color32::BLACK),
                        rounding: Rounding::same(3.0),
                        expansion: 1.0,
                    },
                    active: WidgetVisuals {
                        weak_bg_fill: Color32::from_gray(165),
                        bg_fill: Color32::from_gray(180),
                        bg_stroke: Stroke::new(1.0, Color32::BLACK),
                        fg_stroke: Stroke::new(2.0, Color32::BLACK),
                        rounding: Rounding::same(2.0),
                        expansion: 1.0,
                    },
                    open: WidgetVisuals {
                        weak_bg_fill: Color32::from_gray(220),
                        bg_fill: Color32::from_gray(210),
                        bg_stroke: Stroke::new(1.0, Color32::from_gray(160)),
                        fg_stroke: Stroke::new(1.0, Color32::BLACK),
                        rounding: Rounding::same(2.0),
                        expansion: 0.0,
                    },
                },
                ..Visuals::light()
            },
            ..Default::default()
        });
        Self {
            start_time: Local::now(),
            ..Default::default()
        }
    }
}