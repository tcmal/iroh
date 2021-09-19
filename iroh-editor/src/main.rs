mod app;
mod ffi;
mod message;
mod panes;
mod theme;

use app::App;
use iced::{Sandbox, Settings};

fn main() {
    App::run(Settings::default()).unwrap();
}
