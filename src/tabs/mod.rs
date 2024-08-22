use crate::context::Context;
use crate::tab_viewer::TabUi;
use egui::WidgetText;
use egui_dock::{NodeIndex, SurfaceIndex};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumDiscriminants, EnumIter, IntoEnumIterator};

pub mod log_viewer;
mod tab_a;
mod tab_about;
mod tab_b;

#[derive(Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Serialize, Deserialize, EnumIter, AsRefStr))]
pub enum TabKind {
    TabA(tab_a::TabA),
    TabB(tab_b::TabB),
    TabAbout(tab_about::TabAbout),
    LogViewer(log_viewer::LogViewer),
}

impl TabKindDiscriminants {
    pub fn create_tab(&self, surface: SurfaceIndex, node: NodeIndex) -> Tab {
        let kind = match self {
            TabKindDiscriminants::TabA => TabKind::TabA(tab_a::TabA::default()),
            TabKindDiscriminants::TabB => TabKind::TabB(tab_b::TabB::default()),
            TabKindDiscriminants::TabAbout => TabKind::TabAbout(tab_about::TabAbout::default()),
            TabKindDiscriminants::LogViewer => TabKind::LogViewer(log_viewer::LogViewer::default()),
        };
        Tab {
            surface,
            node,
            kind,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tab {
    pub surface: SurfaceIndex,
    pub node: NodeIndex,
    pub kind: TabKind,
}

impl Tab {
    pub fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Context) {
        match &mut self.kind {
            TabKind::TabA(t) => t.ui(ui, cx),
            TabKind::TabB(t) => t.ui(ui, cx),
            TabKind::TabAbout(t) => t.ui(ui, cx),
            TabKind::LogViewer(t) => t.ui(ui, cx),
        }
    }

    pub fn title(&mut self) -> WidgetText {
        match &mut self.kind {
            TabKind::TabA(t) => t.title(),
            TabKind::TabB(t) => t.title(),
            TabKind::TabAbout(t) => t.title(),
            TabKind::LogViewer(t) => t.title(),
        }
    }

    pub fn add_popup(to: &mut Vec<Tab>, surface: SurfaceIndex, node: NodeIndex, ui: &mut egui::Ui) {
        for tab_kind in TabKindDiscriminants::iter() {
            // Skip log viewer as it needs context to function properly, open through Window -> Log Viewer menu
            if matches!(tab_kind, TabKindDiscriminants::LogViewer) {
                continue;
            }
            if ui.button(tab_kind.as_ref()).clicked() {
                to.push(tab_kind.create_tab(surface, node))
            }
        }
    }

    pub fn default_tabs() -> Vec<Tab> {
        vec![Tab::tab_a(SurfaceIndex::main(), NodeIndex(1))]
    }

    pub fn tab_a(surface: SurfaceIndex, node: NodeIndex) -> Self {
        Tab {
            surface,
            node,
            kind: TabKind::TabA(tab_a::TabA::default()),
        }
    }
}
