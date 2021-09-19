use iced::{pane_grid::Pane, Element, Text};
use iroh_editor::{app::AppState, ffi::AddonPanes, message::Message, panes::Paneable};

pub struct RectWidget;
impl Paneable for RectWidget {
    fn view(&mut self, _pane: Pane, app_state: &AppState) -> Element<Message> {
        Text::new("Hello from RectWidget!")
            .color(app_state.theme().text_accent())
            .into()
    }

    fn title(&self) -> String {
        "Rectangle".to_string()
    }
}

pub struct CircleWidget;
impl Paneable for CircleWidget {
    fn view(&mut self, _pane: Pane, app_state: &AppState) -> Element<Message> {
        Text::new("Hello from CircleWidget!")
            .color(app_state.theme().text_accent())
            .into()
    }

    fn title(&self) -> String {
        "Rectangle".to_string()
    }
}

pub struct MyPanes;
impl AddonPanes for MyPanes {
    fn names(&self) -> Vec<String> {
        vec!["Rectangle".to_string(), "Circle".to_string()]
    }

    fn create(&self, pos: usize) -> Option<Box<dyn Paneable>> {
        match pos {
            0 => Some(Box::new(RectWidget)),
            1 => Some(Box::new(CircleWidget)),
            _ => None,
        }
    }
}
