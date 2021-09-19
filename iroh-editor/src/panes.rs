use iced::{
    button,
    pane_grid::{self, Pane, TitleBar},
    Button, Column, Element, PaneGrid, Row, Text,
};

use crate::{
    app::AppState,
    message::{Message, NewPane, PaneMessage},
};

/// Something which can be displayed in a pane
pub trait Paneable {
    fn view(&mut self, pane: Pane, app_state: &AppState) -> Element<Message>;
    fn title(&self) -> String;
}

/// A layout with a bunch of varying panes, with all the code to split, rearrange, and resize them.
pub struct PaneZone {
    panes: pane_grid::State<PaneState>,
}

impl PaneZone {
    pub fn new(app_state: &AppState) -> Self {
        let (panes, _) = pane_grid::State::new(PaneState::new(Box::new(EmptyPane::new(app_state))));
        Self { panes }
    }

    /// Get what to currently render
    pub fn view(&mut self, app_state: &AppState) -> Element<Message> {
        PaneGrid::new(&mut self.panes, |pane, content| {
            content.view(pane, app_state).into()
        })
        .on_drag(|e| PaneMessage::Drag(e).into())
        .on_resize(10, |e| PaneMessage::Resize(e).into())
        .style(app_state.theme())
        .into()
    }

    /// Process the given message
    pub fn update(&mut self, app_state: &mut AppState, msg: PaneMessage) {
        match msg {
            PaneMessage::Split(axis, pane) => {
                self.panes.split(
                    axis,
                    &pane,
                    PaneState::new(Box::new(EmptyPane::new(app_state))),
                );
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
            PaneMessage::Close(p) => {
                if self.panes.len() > 0 {
                    self.panes.close(&p);
                } else {
                    todo!()
                }
            }
            PaneMessage::Set(p, new) => {
                if let Some(dst) = self.panes.get_mut(&p) {
                    *dst = match new {
                        NewPane::FromSchema(i) => {
                            PaneState::new(app_state.schema().create_pane(i).unwrap())
                        }
                    };
                }
            }
        }
    }
}

/// Wrapper for pane and split controls
pub struct PaneState {
    elem: Box<dyn Paneable>,
    h_state: button::State,
    v_state: button::State,
    c_state: button::State,
}

impl PaneState {
    /// Create a new pane state, with the given content
    pub fn new(elem: Box<dyn Paneable>) -> Self {
        PaneState {
            elem,
            h_state: button::State::new(),
            v_state: button::State::new(),
            c_state: button::State::new(),
        }
    }

    /// Get the contents of the pane
    fn view(&mut self, pane: Pane, app_state: &AppState) -> pane_grid::Content<Message> {
        let controls = Row::with_children(vec![
            Button::new(&mut self.h_state, Text::new("H"))
                .on_press(PaneMessage::Split(pane_grid::Axis::Horizontal, pane).into())
                .style(app_state.theme().button_subtle())
                .into(),
            Button::new(&mut self.v_state, Text::new("V"))
                .on_press(PaneMessage::Split(pane_grid::Axis::Vertical, pane).into())
                .style(app_state.theme().button_subtle())
                .into(),
            Button::new(&mut self.c_state, Text::new("X"))
                .on_press(PaneMessage::Close(pane).into())
                .style(app_state.theme().button_subtle())
                .into(),
        ]);
        let title_bar =
            TitleBar::new(Text::new(self.elem.title()).color(app_state.theme().text_accent()))
                .controls(controls)
                .style(app_state.theme().container_primary());

        let content = self.elem.view(pane, app_state);

        pane_grid::Content::new(content)
            .title_bar(title_bar)
            .style(app_state.theme().container_primary())
    }
}

/// An empty pane, which provides buttons to swap it out for any other pane.
pub struct EmptyPane {
    names: Vec<String>,
    states: Vec<button::State>,
}
impl EmptyPane {
    pub fn new(app_state: &AppState) -> Self {
        let names = app_state.schema().pane_names().to_vec();
        let states = vec![button::State::default(); names.len()];

        Self { names, states }
    }
}
impl Paneable for EmptyPane {
    fn view(&mut self, pane: Pane, app_state: &AppState) -> Element<Message> {
        let mut col = Column::new().padding(10).spacing(10);
        for (i, (name, state)) in self.names.iter().zip(self.states.iter_mut()).enumerate() {
            let btn = Button::new(state, Text::new(name))
                .style(app_state.theme().button_primary())
                .on_press(PaneMessage::Set(pane, NewPane::FromSchema(i)).into());

            col = col.push(btn);
        }

        col.into()
    }

    fn title(&self) -> String {
        "Hello, World!".into()
    }
}
