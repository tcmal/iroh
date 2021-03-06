//! UI logic to deal with splitting & rearranging editor panes.

use crate::{
    app::AppState,
    message::{Message, NewPane, PaneMessage},
    panes::{inspector::InspectorPane, EmptyPane, OutlinePane},
    Kind, ObjectStore,
};
use iced::{
    button,
    pane_grid::{self, Pane, TitleBar},
    Button, Element, PaneGrid, Row, Text,
};

/// Something which can be displayed in a pane
pub trait Paneable<K: Kind, C: ObjectStore<K>> {
    fn view(&mut self, pane: Pane, app_state: &AppState<K, C>) -> Element<Message<K>>;
    fn title(&self) -> String;
}

/// A layout with a bunch of varying panes, with all the code to split, rearrange, and resize them.
pub struct PaneZone<K: Kind, C: ObjectStore<K>> {
    panes: pane_grid::State<PaneState<K, C>>,
}

impl<K: Kind, C: ObjectStore<K>> PaneZone<K, C> {
    /// Create a new pane zone with one [`EmptyPane`]
    pub fn new() -> Self {
        let (panes, _) = pane_grid::State::new(PaneState::new(Box::new(EmptyPane::default())));
        Self { panes }
    }

    /// Get what to currently render
    pub fn view<'b>(&'b mut self, app_state: &AppState<K, C>) -> Element<'b, Message<K>> {
        PaneGrid::new(&mut self.panes, |pane, content| {
            content.view(pane, app_state).into()
        })
        .on_drag(|e| PaneMessage::Drag(e).into())
        .on_resize(10, |e| PaneMessage::Resize(e).into())
        .style(app_state.theme())
        .into()
    }

    /// Process the given message
    pub fn update(&mut self, msg: PaneMessage) {
        match msg {
            PaneMessage::Split(axis, pane) => {
                self.panes
                    .split(axis, &pane, PaneState::new(Box::new(EmptyPane::default())));
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
                        NewPane::Outline => PaneState::new(Box::new(OutlinePane::default())),
                        NewPane::Inspector => {
                            PaneState::new(Box::new(InspectorPane::<K::Field>::default()))
                        }
                    };
                }
            }
        }
    }
}

/// Wrapper for pane with split controls
pub struct PaneState<K: Kind, C: ObjectStore<K>> {
    elem: Box<dyn Paneable<K, C>>,
    h_state: button::State,
    v_state: button::State,
    c_state: button::State,
}

impl<'a, K: Kind, C: ObjectStore<K>> PaneState<K, C> {
    /// Create a new pane state, with the given content
    pub fn new(elem: Box<dyn Paneable<K, C>>) -> Self {
        Self {
            elem,
            h_state: button::State::new(),
            v_state: button::State::new(),
            c_state: button::State::new(),
        }
    }

    /// Get the contents of the pane
    fn view(&mut self, pane: Pane, app_state: &AppState<K, C>) -> pane_grid::Content<Message<K>> {
        // Row of buttons
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

impl<K: Kind, C: ObjectStore<K>> Default for PaneZone<K, C> {
    fn default() -> Self {
        Self::new()
    }
}
