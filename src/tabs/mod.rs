use crate::context::Context;
use crate::prelude::*;
use crate::tab_viewer::TabUi;
use egui::{Ui, WidgetText};
use egui_tiles::{SimplificationOptions, Tile, TileId, Tiles, UiResponse};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use strum::{AsRefStr, EnumDiscriminants, EnumIter};

pub mod tab_a;
pub mod tab_b;

#[derive(Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Serialize, Deserialize, EnumIter, AsRefStr))]
pub enum TabKind {
    TabA(tab_a::TabA),
    TabB(tab_b::TabB),
}

// impl TabKindDiscriminants {
//     pub fn create_tab(&self, nr: usize) -> Tab {
//         let kind = match self {
//             TabKindDiscriminants::TabA => TabKind::TabA(tab_a::TabA::default()),
//             TabKindDiscriminants::TabB => TabKind::TabB(tab_b::TabB::default()),
//         };
//         Tab { kind, nr }
//     }
// }

impl Debug for TabKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TabKind({})", TabKindDiscriminants::from(self).as_ref())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tab {
    pub kind: TabKind,
    pub nr: usize,
}

impl Tab {
    pub fn ui(&mut self, ui: &mut Ui, cx: &mut Context, _title: &str) -> UiResponse {
        // use egui::{Rect, Vec2};
        // use eframe::emath::Align;
        // use egui::{Color32, Layout, Stroke};
        // let color = egui::epaint::Hsva::new(0.103 * self.nr as f32, 0.5, 0.5, 1.0);
        // let color = ui.style().visuals.window_shadow.color;
        // let max_rect = ui.max_rect();
        // let header_rect = Rect::from_min_size(max_rect.min, Vec2::new(max_rect.width(), 20.0));
        // ui.painter().rect_filled(header_rect, 0.0, color);
        // ui.painter().rect(header_rect.shrink(1.0), 0.0, color, Stroke::new(1.0, ui.visuals().faint_bg_color));
        // ui.allocate_ui(header_rect.size(), |ui| {
        //     ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        //         ui.add_space(8.0);
        //         ui.label(title);
        //         ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        //             ui.add_space(8.0);
        //             ui.label("?").on_hover_text("Help text");
        //         })
        //     });
        // });
        // let dragged = ui
        //     .allocate_rect(header_rect, egui::Sense::click_and_drag())
        //     .on_hover_cursor(egui::CursorIcon::Grab)
        //     .dragged();
        egui::Frame::NONE
            .inner_margin(4.0)
            .show(ui, |ui| match &mut self.kind {
                TabKind::TabA(t) => t.ui(ui, cx),
                TabKind::TabB(t) => t.ui(ui, cx),
            });
        // if dragged {
        //     UiResponse::DragStarted
        // } else {
        //     UiResponse::None
        // }
        UiResponse::None
    }

    pub fn title(&self) -> WidgetText {
        match &self.kind {
            TabKind::TabA(t) => t.title(),
            TabKind::TabB(t) => t.title(),
        }
    }

    fn is_closeable(&self) -> bool {
        match &self.kind {
            TabKind::TabA(t) => t.is_closeable(),
            TabKind::TabB(t) => t.is_closeable(),
        }
    }
}

pub struct TreeBehavior {
    simplification_options: egui_tiles::SimplificationOptions,
    tab_bar_height: f32,
    gap_width: f32,
    pub(crate) add_child_to: Option<TileId>,
    cx: Option<Context>,
    show_view_numbers: bool,
}

impl Default for TreeBehavior {
    fn default() -> Self {
        Self {
            simplification_options: SimplificationOptions {
                prune_empty_tabs: true,
                prune_empty_containers: true,
                prune_single_child_tabs: false,
                prune_single_child_containers: false,
                all_panes_must_have_tabs: true,
                join_nested_linear_containers: false,
            },
            tab_bar_height: 20.0,
            gap_width: 2.0,
            add_child_to: None,
            cx: None,
            show_view_numbers: false,
        }
    }
}

impl TreeBehavior {
    pub(crate) fn ui(&mut self, ui: &mut Ui) {
        let Self {
            simplification_options,
            tab_bar_height,
            gap_width,
            add_child_to: _,
            cx: _,
            show_view_numbers: _,
        } = self;

        egui::Grid::new("behavior_ui")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("All panes must have tabs:");
                ui.checkbox(&mut simplification_options.all_panes_must_have_tabs, "");
                ui.end_row();

                ui.label("Join nested containers:");
                ui.checkbox(
                    &mut simplification_options.join_nested_linear_containers,
                    "",
                );
                ui.end_row();

                ui.label("Tab bar height:");
                ui.add(
                    egui::DragValue::new(tab_bar_height)
                        .range(0.0..=100.0)
                        .speed(1.0),
                );
                ui.end_row();

                ui.label("Gap width:");
                ui.add(egui::DragValue::new(gap_width).range(0.0..=20.0).speed(1.0));
                ui.end_row();

                ui.label("Show view numbers:");
                ui.checkbox(&mut self.show_view_numbers, "");
                ui.end_row();
            });
    }

    pub fn feed_cx(&mut self, cx: Context) {
        self.cx = Some(cx);
    }
}

impl egui_tiles::Behavior<Tab> for TreeBehavior {
    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, view: &mut Tab) -> UiResponse {
        if let Some(cx) = &mut self.cx {
            view.ui(ui, cx, view.title().text())
        } else {
            UiResponse::None
        }
    }

    fn tab_title_for_pane(&mut self, view: &Tab) -> WidgetText {
        if self.show_view_numbers {
            format!("{}: {}", view.nr, view.title().text()).into()
        } else {
            view.title()
        }
    }

    fn is_tab_closable(&self, tiles: &Tiles<Tab>, tile_id: TileId) -> bool {
        if let Some(tile) = tiles.get(tile_id) {
            match tile {
                Tile::Pane(tab) => tab.is_closeable(),
                Tile::Container(_) => false,
            }
        } else {
            true
        }
    }

    // ---
    // Settings:

    fn on_tab_close(&mut self, tiles: &mut Tiles<Tab>, tile_id: TileId) -> bool {
        if let Some(tile) = tiles.get(tile_id) {
            match tile {
                Tile::Pane(pane) => {
                    // Single pane removal
                    let tab_title = self.tab_title_for_pane(pane);
                    debug!("Closing tab: {}, tile ID: {tile_id:?}", tab_title.text());
                }
                Tile::Container(container) => {
                    // Container removal
                    debug!("Closing container: {:?}", container.kind());
                    let children_ids = container.children();
                    for child_id in children_ids {
                        if let Some(Tile::Pane(pane)) = tiles.get(*child_id) {
                            let tab_title = self.tab_title_for_pane(pane);
                            debug!("Closing tab: {}, tile ID: {tile_id:?}", tab_title.text());
                        }
                    }
                }
            }
        }

        // Proceed to removing the tab
        true
    }

    fn top_bar_right_ui(
        &mut self,
        _tiles: &Tiles<Tab>,
        ui: &mut Ui,
        tile_id: TileId,
        _tabs: &egui_tiles::Tabs,
        _scroll_offset: &mut f32,
    ) {
        ui.add_space(4.0);
        if ui.button("➕").clicked() {
            self.add_child_to = Some(tile_id);
        }
    }

    fn tab_bar_height(&self, _style: &egui::Style) -> f32 {
        self.tab_bar_height
    }

    fn gap_width(&self, _style: &egui::Style) -> f32 {
        self.gap_width
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        self.simplification_options
    }
}
