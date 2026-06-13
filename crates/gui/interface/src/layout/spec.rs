/// Declarative page layout (like HTML structure).
///
/// Positions use fractions of the window (0.0–1.0) so elements stay in the same
/// relative place when the window is resized.
#[derive(Clone, Debug)]
pub struct Layout {
    pub items: Vec<LayoutItem>,
}

#[derive(Clone, Debug)]
pub struct LayoutItem {
    pub element: Element,
    /// Horizontal center, 0 = left edge, 1 = right edge.
    pub x_pct: f32,
    /// Vertical center, 0 = top edge, 1 = bottom edge.
    pub y_pct: f32,
    /// Width as fraction of window width.
    pub width_pct: f32,
    /// Height as fraction of window height.
    pub height_pct: f32,
}

#[derive(Clone, Debug)]
pub enum Element {
    Text {
        content: String,
        scale: f32,
    },
    Checkbox {
        label: String,
        id: String,
    },
    Button {
        label: String,
        id: String,
        /// When set, the button is only enabled if this checkbox id is checked.
        requires: Option<String>,
    },
}

impl Layout {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn text(
        mut self,
        content: impl Into<String>,
        x_pct: f32,
        y_pct: f32,
    ) -> Self {
        self.items.push(LayoutItem {
            element: Element::Text {
                content: content.into(),
                scale: 1.0,
            },
            x_pct,
            y_pct,
            width_pct: 0.9,
            height_pct: 0.1,
        });
        self
    }

    pub fn text_scaled(
        mut self,
        content: impl Into<String>,
        x_pct: f32,
        y_pct: f32,
        scale: f32,
    ) -> Self {
        self.items.push(LayoutItem {
            element: Element::Text {
                content: content.into(),
                scale,
            },
            x_pct,
            y_pct,
            width_pct: 0.9,
            height_pct: 0.12,
        });
        self
    }

    pub fn checkbox(
        mut self,
        label: impl Into<String>,
        id: impl Into<String>,
        x_pct: f32,
        y_pct: f32,
    ) -> Self {
        self.items.push(LayoutItem {
            element: Element::Checkbox {
                label: label.into(),
                id: id.into(),
            },
            x_pct,
            y_pct,
            width_pct: 0.6,
            height_pct: 0.08,
        });
        self
    }

    pub fn button(
        mut self,
        label: impl Into<String>,
        id: impl Into<String>,
        x_pct: f32,
        y_pct: f32,
    ) -> Self {
        self.items.push(LayoutItem {
            element: Element::Button {
                label: label.into(),
                id: id.into(),
                requires: None,
            },
            x_pct,
            y_pct,
            width_pct: 0.35,
            height_pct: 0.08,
        });
        self
    }

    pub fn button_requires(
        mut self,
        label: impl Into<String>,
        id: impl Into<String>,
        requires: impl Into<String>,
        x_pct: f32,
        y_pct: f32,
    ) -> Self {
        self.items.push(LayoutItem {
            element: Element::Button {
                label: label.into(),
                id: id.into(),
                requires: Some(requires.into()),
            },
            x_pct,
            y_pct,
            width_pct: 0.35,
            height_pct: 0.08,
        });
        self
    }
}
