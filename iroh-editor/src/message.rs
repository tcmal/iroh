use iced::pane_grid;
use iroh::Key;

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

impl<K: Key> Into<Message<K>> for PaneMessage {
    fn into(self) -> Message<K> {
        Message::PaneMessage(self)
    }
}

/// Root message type for our app.
#[derive(Debug, Clone, Copy)]
pub enum Message<K: Key> {
    PaneMessage(PaneMessage),
    Select(K),
}
