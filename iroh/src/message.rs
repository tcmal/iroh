//! The message types used throughout the application

use crate::{mutation::Mutator, Kind};
use iced::pane_grid;

/// A message related to a pane zone.
#[derive(Debug, Clone, Copy)]
pub enum PaneMessage {
    Split(pane_grid::Axis, pane_grid::Pane),
    Drag(pane_grid::DragEvent),
    Resize(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    Set(pane_grid::Pane, NewPane),
}

/// Which pane to open
#[derive(Debug, Clone, Copy)]
pub enum NewPane {
    Outline,
    Inspector,
}

impl<K: Kind> Into<Message<K>> for PaneMessage {
    fn into(self) -> Message<K> {
        Message::PaneMessage(self)
    }
}

/// Root message type for our app.
#[derive(Debug, Clone)]
pub enum Message<K: Kind> {
    PaneMessage(PaneMessage),
    Select(K::Key),
    NewObject,
    Mutate(Box<dyn Mutator<K>>, Box<dyn Mutator<K::WorkingValues>>),
    Nop,
}
