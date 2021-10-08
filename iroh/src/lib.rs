#![allow(incomplete_features)]
#![feature(generic_associated_types, const_generics)]

pub mod app;
pub mod fields;
pub mod kinds;
pub mod lens;
pub mod message;
pub mod mutation;
mod pane_zone;
mod panes;
pub mod stores;
mod theme;

pub use app::App;
pub use iced::{Sandbox, Settings};
pub use kinds::{Field, Key, Kind};
pub use message::Message;
pub use stores::ObjectStore;
pub use theme::Theme;
