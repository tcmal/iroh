use iced::{button, container, pane_grid, Background, Color, Vector};

/// Represents the currently in-use theme. This provides a way to get colours semantically, as in by their purpose.
#[derive(Debug, Clone)]
pub enum Theme {
    Dark,
}

impl Theme {
    /// To be used for text intended to be subtle, such as help text, when on bg_primary
    pub fn text_subtle(&self) -> Color {
        dark::TEXT_SUBTLE
    }

    /// To be used for the bulk of text, when on bg_primary
    pub fn text_primary(&self) -> Color {
        dark::TEXT_PRIMARY
    }

    /// To be used for text that should stand out, when on bg_primary
    pub fn text_accent(&self) -> Color {
        dark::TEXT_ACCENT
    }

    /// To be used for text displayed on bg_accent
    pub fn text_on_accent(&self) -> Color {
        dark::TEXT_ON_ACCENT
    }

    /// To be used for most areas
    pub fn bg_primary(&self) -> Color {
        dark::BACKGROUND_PRIMARY
    }

    /// To be used for areas that should stand out
    pub fn bg_accent(&self) -> Color {
        dark::BACKGROUND_ACCENT
    }

    /// Style for most containers
    pub fn container_primary(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Dark => ContainerStyle {
                text: dark::TEXT_PRIMARY,
                bg: dark::BACKGROUND_PRIMARY,
            }
            .into(),
        }
    }

    /// Style for containers that should stand out
    pub fn container_accent(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Dark => ContainerStyle {
                text: dark::TEXT_ON_ACCENT,
                bg: dark::BACKGROUND_ACCENT,
            }
            .into(),
        }
    }

    /// Style for buttons that shouldn't stand out too much, such as auxillary functions
    pub fn button_subtle(&self) -> Box<dyn button::StyleSheet> {
        match self {
            Theme::Dark => ButtonStyle {
                text: dark::TEXT_ACCENT,
                bg: dark::BACKGROUND_PRIMARY,
            }
            .into(),
        }
    }

    /// Style for buttons that should stand out, such as 'Next'
    pub fn button_primary(&self) -> Box<dyn button::StyleSheet> {
        match self {
            Theme::Dark => ButtonStyle {
                text: dark::TEXT_ON_ACCENT,
                bg: dark::BACKGROUND_ACCENT,
            }
            .into(),
        }
    }
}

impl Into<Box<dyn pane_grid::StyleSheet>> for &Theme {
    fn into(self) -> Box<dyn pane_grid::StyleSheet> {
        match self {
            Theme::Dark => PaneGridStyle(dark::TEXT_ACCENT),
        }
        .into()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

/// A style we apply to containers
struct ContainerStyle {
    text: Color,
    bg: Color,
}
impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(self.text),
            background: Some(Background::Color(self.bg)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: self.bg,
        }
    }
}

/// A style we apply to buttons
struct ButtonStyle {
    bg: Color,
    text: Color,
}
impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(self.bg)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: self.bg,
            text_color: self.text,
        }
    }
}

/// A style we apply to pane grids.
struct PaneGridStyle (Color);
impl pane_grid::StyleSheet for PaneGridStyle {
    fn picked_split(&self) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: self.0,
            width: 2.0,
        })
    }

    fn hovered_split(&self) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: self.0,
            width: 1.0,
        })
    }
}

mod dark {
    use iced::Color;

    pub const TEXT_SUBTLE: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);
    pub const TEXT_PRIMARY: Color = Color::WHITE;
    pub const TEXT_ACCENT: Color =
        Color::from_rgba(0.39215686274, 0.86666666666, 0.09019607843, 1.0);
    pub const TEXT_ON_ACCENT: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.5);
    pub const BACKGROUND_PRIMARY: Color =
        Color::from_rgba(0.1294117647, 0.1294117647, 0.1294117647, 1.0);
    pub const BACKGROUND_ACCENT: Color =
        Color::from_rgba(0.10588235294, 0.36862745098, 0.12549019607, 1.0);
}
