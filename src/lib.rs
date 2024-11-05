#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod context;
mod prelude;
mod tab_viewer;
mod tabs;
mod windows;

pub use app::TemplateApp;
