#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod context;
mod prelude;
mod tab_viewer;
mod tabs;

pub use app::TemplateApp;
