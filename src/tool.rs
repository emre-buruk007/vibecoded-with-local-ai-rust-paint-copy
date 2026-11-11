#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Pencil,
    Line,
    Rectangle,
    Ellipse,
    Fill,
    Eyedropper,
    Eraser,
}

impl Tool {
    pub fn all() -> [Self; 7] {
        [Self::Pencil, Self::Line, Self::Rectangle, Self::Ellipse, Self::Fill, Self::Eyedropper, Self::Eraser]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Pencil => "Pencil",
            Self::Line => "Line",
            Self::Rectangle => "Rectangle",
            Self::Ellipse => "Ellipse",
            Self::Fill => "Fill",
            Self::Eyedropper => "Eyedropper",
            Self::Eraser => "Eraser",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Pencil => "✏️",
            Self::Line => "📏",
            Self::Rectangle => "⬜",
            Self::Ellipse => "⭕",
            Self::Fill => "🪣",
            Self::Eyedropper => "💧",
            Self::Eraser => "🧹",
        }
    }

    pub fn is_shape(&self) -> bool {
        matches!(self, Self::Line | Self::Rectangle | Self::Ellipse)
    }
}
