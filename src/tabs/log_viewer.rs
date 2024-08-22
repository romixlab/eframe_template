use crate::prelude::*;
use egui_tracing::EventCollector;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogViewer {
    #[serde(skip)]
    event_collector: EventCollector,
}

impl TabUi for LogViewer {
    fn title(&self) -> WidgetText {
        "Log Viewer".into()
    }

    fn ui(&mut self, ui: &mut Ui, _cx: &mut Context) {
        ui.add(egui_tracing::Logs::new(self.event_collector.clone()));
    }
}

impl LogViewer {
    pub fn new(event_collector: EventCollector) -> Self {
        Self { event_collector }
    }
}
