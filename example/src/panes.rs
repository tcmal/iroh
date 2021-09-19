use iced::{pane_grid::Pane, Element, Text};
use iroh_editor::{ffi::AddonPanes, message::Message, panes::Paneable, theme::Theme};

pub struct RectWidget;
impl Paneable for RectWidget {
    fn view(&mut self, pane: Pane, theme: &Theme) -> Element<Message> {
        Text::new("Hello from RectWidget!")
            .color(theme.text_accent())
            .into()
    }

    fn title(&self) -> String {
        "Rectangle".to_string()
    }
}

pub struct CircleWidget;
impl Paneable for CircleWidget {
    fn view(&mut self, pane: Pane, theme: &Theme) -> Element<Message> {
        Text::new("Hello from CircleWidget!")
            .color(theme.text_accent())
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
