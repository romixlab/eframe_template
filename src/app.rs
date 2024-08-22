use egui::{TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use crate::context::Context;
use crate::tab_viewer::AppTabViewer;
use crate::tabs::{Tab, TabKindDiscriminants};

pub struct TemplateApp {
    cx: Context,
    state: State,
    shutdown_event_tx: Option<oneshot::Sender<()>>
}

#[derive(Serialize, Deserialize)]
struct State {
    dock_state: DockState<Tab>,
    tab_counter: usize,
}

impl Default for State {
    fn default() -> Self {
        let tabs = Tab::default_tabs();
        let tab_counter = tabs.len() + 1;
        let dock_state = DockState::new(tabs);
        State {
            dock_state,
            tab_counter,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, cx: Context, shutdown_event_tx: oneshot::Sender<()>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            let state = eframe::get_value(storage, eframe::APP_KEY).unwrap_or(State::default());
            TemplateApp { cx, state, shutdown_event_tx: Some(shutdown_event_tx) }
        } else {
            let state = State::default();
            TemplateApp { cx, state, shutdown_event_tx: Some(shutdown_event_tx) }
        }
        // if let Some(storage) = cc.storage {
        //     // Restore context for all the tabs
        //     for (_, tab) in state.dock_state.iter_all_tabs_mut() {
        //         tab.load_state(storage, &mut cx);
        //     }
        // }
    }

    fn menu_bar(&mut self, ui: &mut Ui) {
        let is_web = cfg!(target_arch = "wasm32");
        ui.menu_button("File", |ui| {
            // NOTE: no File->Quit on web pages!
            if !is_web {
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
        ui.menu_button("Window", |ui| {
            if ui.button("Tab A").clicked() {
                self.state.new_tab_window(TabKindDiscriminants::TabA);
            }
        });
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                self.state.new_tab_window(TabKindDiscriminants::TabAbout);
            }
        });
        ui.add_space(16.0);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::warn_if_debug_build(ui);
            egui::widgets::global_dark_light_mode_buttons(ui);
        });
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.menu_bar(ui);
            });
        });

        let mut added_nodes = Vec::new();
        DockArea::new(&mut self.state.dock_state)
            .show_add_buttons(true)
            .show_add_popup(true)
            .style({
                let mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
                style.tab_bar.fill_tab_bar = true;
                style.tab_bar.height = 20.0;
                style
            })
            .show(
                ctx,
                &mut AppTabViewer {
                    added_nodes: &mut added_nodes,
                    cx: &mut self.cx,
                },
            );

        added_nodes.drain(..).for_each(|node| {
            self.state
                .dock_state
                .set_focused_node_and_surface((node.surface, node.node));
            self.state.dock_state.push_to_focused_leaf(Tab {
                kind: node.kind,
                surface: node.surface,
                node: NodeIndex(self.state.tab_counter),
            });
            self.state.tab_counter += 1;
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    // fn on_exit(&mut self) {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(tx) = self.shutdown_event_tx.take() {
            _ = tx.send(());
        }
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }
}

impl State {
    fn new_tab_window(&mut self, kind: TabKindDiscriminants) {
        let tab = kind.create_tab(SurfaceIndex::main(), NodeIndex(self.tab_counter));
        self.tab_counter += 1;
        let new_surface_idx = self.dock_state.add_window(vec![tab]);
        for ((surface_idx, _node_idx), tab) in self.dock_state.iter_all_tabs_mut() {
            if surface_idx == new_surface_idx {
                tab.surface = new_surface_idx;
            }
        }
    }
}