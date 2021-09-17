use iced::{
    button,
    pane_grid::{self, Pane, TitleBar},
    Button, Element, PaneGrid, Row, Text,
};

use crate::message::{Message, PaneMessage};

/// A layout with a bunch of varying panes, with all the code to split, rearrange, and resize them.
pub struct PaneZone {
    panes: pane_grid::State<PaneState>,
}

impl PaneZone {
    pub fn view(&mut self) -> Element<Message> {
        PaneGrid::new(&mut self.panes, |pane, content| content.view(pane).into())
            .on_drag(|e| PaneMessage::Drag(e).into())
            .on_resize(10, |e| PaneMessage::Resize(e).into())
            .into()
    }
    pub fn update(&mut self, msg: PaneMessage) {
        match msg {
            PaneMessage::Split(axis, pane) => {
                self.panes.split(axis, &pane, PaneState::default());
            }
            PaneMessage::Resize(e) => {
                self.panes.resize(&e.split, e.ratio);
            }
            PaneMessage::Drag(e) => match e {
                pane_grid::DragEvent::Dropped { pane, target } => {
                    self.panes.swap(&pane, &target);
                }
                _ => (),
            },
        }
    }
}
impl Default for PaneZone {
    fn default() -> Self {
        let (panes, _) = pane_grid::State::new(PaneState::default());
        Self { panes }
    }
}

/// Something which can be displayed in a pane
pub trait Paneable {
    fn view(&mut self, pane: Pane) -> Element<Message>;
    fn title(&self) -> String;
}

/// Wrapper for pane and split controls
pub struct PaneState {
    elem: Box<dyn Paneable>,
    h_state: button::State,
    v_state: button::State,
}

impl PaneState {
    /// Create a new pane state, with the given content
    pub fn new(elem: Box<dyn Paneable>) -> Self {
        PaneState {
            elem,
            h_state: button::State::new(),
            v_state: button::State::new(),
        }
    }

    /// Get the contents of the pane
    fn view(&mut self, pane: Pane) -> pane_grid::Content<Message> {
        let controls = Row::with_children(vec![
            Button::new(&mut self.h_state, Text::new("H"))
                .on_press(PaneMessage::Split(pane_grid::Axis::Horizontal, pane).into())
                .style(style::ControlButtons)
                .into(),
            Button::new(&mut self.v_state, Text::new("V"))
                .on_press(PaneMessage::Split(pane_grid::Axis::Vertical, pane).into())
                .style(style::ControlButtons)
                .into(),
        ]);
        let title_bar = TitleBar::new(Text::new(self.elem.title()))
            .controls(controls)
            .style(style::TitleBarStyle);

        let content = self.elem.view(pane);

        pane_grid::Content::new(content)
            .title_bar(title_bar)
            .style(style::ContentStyle)
    }
}

impl Default for PaneState {
    fn default() -> Self {
        PaneState {
            elem: Box::new(EmptyPane),
            h_state: button::State::new(),
            v_state: button::State::new(),
        }
    }
}

/// An empty pane, that just says hello. This is the default for any new split.
pub struct EmptyPane;
impl Paneable for EmptyPane {
    fn view(&mut self, pane: Pane) -> Element<Message> {
        Text::new(format!("Hello from pane {:?}", pane))
            .color(crate::style::COLOR_TEXT)
            .size(16)
            .into()
    }

    fn title(&self) -> String {
        "Hello, World!".into()
    }
}

mod style {
    use crate::style::*;
    use iced::{button, widget::container, Background, Color, Vector};

    pub struct TitleBarStyle;
    impl container::StyleSheet for TitleBarStyle {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(COLOR_TEXT),
                background: Some(Background::Color(COLOR_BACKGROUND)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }
    }

    pub struct ContentStyle;
    impl container::StyleSheet for ContentStyle {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(COLOR_TEXT),
                background: Some(Background::Color(COLOR_BACKGROUND)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }
    }
    pub struct ControlButtons;
    impl button::StyleSheet for ControlButtons {
        fn active(&self) -> button::Style {
            button::Style {
                shadow_offset: Vector::new(0.0, 0.0),
                background: Some(Background::Color(COLOR_TEXT)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: COLOR_BACKGROUND,
            }
        }
    }
}
