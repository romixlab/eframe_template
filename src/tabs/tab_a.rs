use crate::context::Context;
use crate::tab_viewer::TabUi;
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct TabA {
    s: String,
}

impl TabUi for TabA {
    fn title(&self) -> WidgetText {
        "Tab A".into()
    }

    fn ui(&mut self, ui: &mut Ui, _cx: &mut Context) {
        ui.text_edit_singleline(&mut self.s);
    }
}
