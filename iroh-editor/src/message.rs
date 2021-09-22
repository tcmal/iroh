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

impl Into<Message> for PaneMessage {
    fn into(self) -> Message {
        Message::PaneMessage(self)
    }
}

/// Root message type for our app.
#[derive(Debug, Clone, Copy)]
pub enum Message {
    PaneMessage(PaneMessage),
}
