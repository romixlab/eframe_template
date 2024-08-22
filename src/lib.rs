#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod context;
mod tab_viewer;
mod tabs;
mod prelude;

pub use app::TemplateApp;
