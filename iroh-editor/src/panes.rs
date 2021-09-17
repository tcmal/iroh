use iced::{
    button,
    pane_grid::{self, Pane, TitleBar},
    Button, Element, PaneGrid, Row, Text,
};

use crate::{
    message::{Message, PaneMessage},
    theme::Theme,
};

/// A layout with a bunch of varying panes, with all the code to split, rearrange, and resize them.
pub struct PaneZone {
    panes: pane_grid::State<PaneState>,
}

impl PaneZone {
    /// Get what to currently render
    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        PaneGrid::new(&mut self.panes, |pane, content| {
            content.view(pane, theme).into()
        })
        .on_drag(|e| PaneMessage::Drag(e).into())
        .on_resize(10, |e| PaneMessage::Resize(e).into())
        .style(theme)
        .into()
    }

    /// Process the given message
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
    fn view(&mut self, pane: Pane, theme: &Theme) -> Element<Message>;
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
    fn view(&mut self, pane: Pane, theme: &Theme) -> pane_grid::Content<Message> {
        let controls = Row::with_children(vec![
            Button::new(&mut self.h_state, Text::new("H"))
                .on_press(PaneMessage::Split(pane_grid::Axis::Horizontal, pane).into())
                .style(theme.button_primary())
                .into(),
            Button::new(&mut self.v_state, Text::new("V"))
                .on_press(PaneMessage::Split(pane_grid::Axis::Vertical, pane).into())
                .style(theme.button_primary())
                .into(),
        ]);
        let title_bar = TitleBar::new(Text::new(self.elem.title()).color(theme.text_accent()))
            .controls(controls)
            .style(theme.container_primary());

        let content = self.elem.view(pane, theme);

        pane_grid::Content::new(content)
            .title_bar(title_bar)
            .style(theme.container_primary())
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
    fn view(&mut self, pane: Pane, theme: &Theme) -> Element<Message> {
        Text::new(format!("Hello from pane {:?}", pane))
            .color(theme.text_primary())
            .size(16)
            .into()
    }

    fn title(&self) -> String {
        "Hello, World!".into()
    }
}
