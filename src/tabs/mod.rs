use crate::context::Context;
use crate::tab_viewer::TabUi;
use egui::WidgetText;
use egui_dock::{NodeIndex, SurfaceIndex};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumDiscriminants, EnumIter, IntoEnumIterator};

mod tab_a;
mod tab_about;
mod tab_b;

#[derive(Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Serialize, Deserialize, EnumIter, AsRefStr))]
pub enum TabKind {
    TabA(tab_a::TabA),
    TabB(tab_b::TabB),
    TabAbout(tab_about::TabAbout),
}

impl TabKindDiscriminants {
    pub fn create_tab(&self, surface: SurfaceIndex, node: NodeIndex) -> Tab {
        let kind = match self {
            TabKindDiscriminants::TabA => TabKind::TabA(tab_a::TabA::default()),
            TabKindDiscriminants::TabB => TabKind::TabB(tab_b::TabB::default()),
            TabKindDiscriminants::TabAbout => TabKind::TabAbout(tab_about::TabAbout::default()),
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
        }
    }

    pub fn title(&mut self) -> WidgetText {
        match &mut self.kind {
            TabKind::TabA(t) => t.title(),
            TabKind::TabB(t) => t.title(),
            TabKind::TabAbout(t) => t.title(),
        }
    }

    pub fn add_popup(to: &mut Vec<Tab>, surface: SurfaceIndex, node: NodeIndex, ui: &mut egui::Ui) {
        for tab_kind in TabKindDiscriminants::iter() {
            if ui.button(tab_kind.as_ref()).clicked() {
                to.push(tab_kind.create_tab(surface, node))
            }
        }
    }

    // fn from_ron(map: ron::Map, cx: &mut Context) -> Result<Tab, ron::Error> {
    //     let mut kind: Option<TabKindDiscriminants> = None;
    //     let mut state: Option<String> = None;
    //     let mut surface = None;
    //     let mut node: Option<NodeIndex> = None;
    //     for (k, v) in map.into_iter() {
    //         let Value::String(k) = k else {
    //             continue
    //         };
    //         match k.as_str() {
    //             "kind" => {
    //                 let Value::String(v) = v else {
    //                     continue
    //                 };
    //                 kind = Some(ron::from_str(v.as_str())?);
    //             }
    //             "state" => {
    //                 let Value::String(v) = v else {
    //                     continue
    //                 };
    //                 state = Some(v);
    //             }
    //             "surface" => {
    //                 let Value::Number(v) = v else {
    //                     continue
    //                 };
    //                 let Number::Integer(v) = v else {
    //                     continue
    //                 };
    //                 surface = Some(SurfaceIndex(v as usize));
    //             }
    //             "node" => {
    //                 let Value::Number(v) = v else {
    //                     continue
    //                 };
    //                 let Number::Integer(v) = v else {
    //                     continue
    //                 };
    //                 node = Some(NodeIndex(v as usize));
    //             }
    //             _ => {
    //                 return Err(serde::de::Error::custom("Invalid Tab field"));
    //             }
    //         }
    //     }
    //
    //     let (Some(kind), Some(state), Some(surface), Some(node)) = (kind, state, surface, node) else {
    //         return Err(serde::de::Error::custom("Missing Tab fields"));
    //     };
    //
    //     Ok(kind.create_tab(surface, node))
    // }

    // pub fn to_ron(&self) -> Value {
    //     let mut map = ron::Map::new();
    //     map.insert(Value::String("kind".into()), Value::String(ron::to_string(&TabKindDiscriminants::from(&self.kind)).unwrap()));
    //     map.insert(Value::String("state".into()), Value::String("tab state goes here".into()));
    //     map.insert(Value::String("surface".into()), Value::Number(Number::Integer(self.surface.0 as i64)));
    //     map.insert(Value::String("node".into()), Value::Number(Number::Integer(self.node.0 as i64)));
    //     Value::Map(map)
    // }

    // pub fn tabs_from_ron(tabs: Value, cx: &mut Context) -> Vec<Tab> {
    //     if let Value::Seq(tab_states) = tabs {
    //         let mut tabs = vec![];
    //         for tab_state in tab_states.into_iter() {
    //             let Value::Map(tab_state) = tab_state else {
    //                 continue
    //             };
    //             if let Ok(tab) = Self::from_ron(tab_state, cx) {
    //                 tabs.push(tab);
    //             }
    //         }
    //         tabs
    //     } else {
    //         Self::default_tabs()
    //     }
    // }

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

// pub struct TabSeed<'a> {
//     cx: &'a mut Context
// }

// impl<'a, 'de> Deserialize<'de> for TabSeed<'a> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_map(TabVisitor { cx: self.cx })
//     }
// }

// struct TabVisitor<'a> {
//     cx: &'a mut Context
// }
//
// impl<'a> TabVisitor<'a> {
//
// }

// impl<'a, 'de> Visitor<'de> for TabVisitor<'a> {
//     type Value = Tab;
//
//     fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "a map with Tab fields as keys")
//     }
//
//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//     where
//         A: MapAccess<'de>,
//     {
//     }
// }
