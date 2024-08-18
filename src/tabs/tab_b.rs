use crate::context::Context;
use crate::tab_viewer::TabUi;
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct TabB {
    a: f32,
}

impl TabUi for TabB {
    fn title(&self) -> WidgetText {
        "Tab B".into()
    }

    fn ui(&mut self, ui: &mut Ui, _cx: &mut Context) {
        ui.add(egui::Slider::new(&mut self.a, 0.0..=10.0));
    }
}
