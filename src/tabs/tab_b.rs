use crate::prelude::*;

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

        if ui.button("Log").clicked() {
            info!("Button in TabB");
        }
    }

    fn is_closeable(&self) -> bool {
        true
    }
}
