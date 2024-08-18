use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::tab_viewer::TabUi;

#[derive(Default, Serialize, Deserialize)]
pub struct TabAbout {

}

impl TabUi for TabAbout {
    fn title(&self) -> WidgetText {
        "About".into()
    }

    fn ui(&mut self, ui: &mut Ui, _cx: &mut Context) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(".");
        });
    }
}