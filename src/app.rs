use crate::context::Context;
use crate::tab_viewer::AppTabViewer;
use crate::tabs::{Tab, TabKind, TabKindDiscriminants};
use egui::{TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex};
use egui_modal::Modal;
use egui_tracing::EventCollector;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

pub struct TemplateApp {
    cx: Context,
    state: State,
    log_viewer: crate::tabs::log_viewer::LogViewer,
    shutdown_event_tx: Option<oneshot::Sender<()>>,
    shutdown_modal: Modal,
    shutdown_confirmed: bool,
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
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        cx: Context,
        event_collector: EventCollector,
        shutdown_event_tx: oneshot::Sender<()>,
    ) -> Self {
        // Load previous app state (if any).
        let mut state = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or(State::default())
        } else {
            State::default()
        };

        let log_viewer = crate::tabs::log_viewer::LogViewer::new(event_collector);
        // restore Log Viewer context if it's window is already open
        for (_, tab) in state.dock_state.iter_all_tabs_mut() {
            if matches!(tab.kind, TabKind::LogViewer(_)) {
                tab.kind = TabKind::LogViewer(log_viewer.clone());
            }
        }

        TemplateApp {
            cx,
            state,
            log_viewer,
            shutdown_event_tx: Some(shutdown_event_tx),
            shutdown_modal: Modal::new(&cc.egui_ctx, "shutdown_modal"),
            shutdown_confirmed: false,
        }
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
            if ui.button("Log Viewer").clicked() {
                if !self.state.focus_on_tab(TabKindDiscriminants::LogViewer) {
                    self.state
                        .new_tab(TabKind::LogViewer(self.log_viewer.clone()));
                }
                ui.close_menu();
            }
            if ui.button("Tab A").clicked() {
                self.state.new_tab_default(TabKindDiscriminants::TabA);
                ui.close_menu();
            }
        });
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                if !self.state.focus_on_tab(TabKindDiscriminants::TabAbout) {
                    self.state.new_tab_default(TabKindDiscriminants::TabAbout);
                }
                ui.close_menu();
            }
        });
        ui.add_space(16.0);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::warn_if_debug_build(ui);
            egui::widgets::global_theme_preference_buttons(ui);
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

        let modal = &self.shutdown_modal;
        if ctx.input(|i| i.viewport().close_requested()) {
            if !self.shutdown_confirmed {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                modal.open();
            }
        }
        modal.show(|ui| {
            modal.title(ui, "Confirm exit");
            modal.frame(ui, |ui| {
                ui.label("Are you sure you want to exit?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "Cancel");
                if modal.suggested_button(ui, "Save & Exit").clicked() {
                    // TODO: Save things, set shutdown_confirmed to true and send a Close command
                };
                if modal.caution_button(ui, "Discard & Exit").clicked() {
                    self.shutdown_confirmed = true;
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                };
            });
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
    fn new_tab_default(&mut self, kind: TabKindDiscriminants) {
        let tab = kind.create_tab(SurfaceIndex::main(), NodeIndex(self.tab_counter));
        self.add_tab(tab);
    }

    fn new_tab(&mut self, kind: TabKind) {
        let tab = Tab {
            surface: SurfaceIndex::main(),
            node: NodeIndex(self.tab_counter),
            kind,
        };
        self.add_tab(tab);
    }

    fn add_tab(&mut self, tab: Tab) {
        self.tab_counter += 1;
        let new_surface_idx = self.dock_state.add_window(vec![tab]);
        for ((surface_idx, _node_idx), tab) in self.dock_state.iter_all_tabs_mut() {
            if surface_idx == new_surface_idx {
                tab.surface = new_surface_idx;
            }
        }
    }

    /// Focus on a first found tab with specified kind.
    /// Returns false if no such tab were found.
    fn focus_on_tab(&mut self, tab_kind: TabKindDiscriminants) -> bool {
        let mut surface_node_tab = None;
        for (surface_index, surface) in self.dock_state.iter_surfaces().enumerate() {
            for (tab_index, (node, tab)) in surface.iter_all_tabs().enumerate() {
                if TabKindDiscriminants::from(&tab.kind) == tab_kind {
                    surface_node_tab = Some((
                        SurfaceIndex(surface_index),
                        node,
                        egui_dock::TabIndex(tab_index),
                    ));
                    break;
                }
            }
        }
        if let Some(surface_node_tab) = surface_node_tab {
            self.dock_state.set_active_tab(surface_node_tab);
            true
        } else {
            false
        }
    }
}
