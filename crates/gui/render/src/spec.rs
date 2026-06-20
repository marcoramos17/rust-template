/// Dimension length unit. Supports combinations of % and px.
#[derive(Clone, Debug, Copy)]
pub struct Length {
    pub percent: f32,
    pub pixels: f32,
}

/// Positioning unit relative to container edges.
#[derive(Clone, Debug, Copy)]
pub enum Position {
    FromStart(Length),
    FromEnd(Length),
    Center(Length),
}

impl Length {
    pub const ZERO: Self = Self { percent: 0.0, pixels: 0.0 };

    pub fn px(pixels: f32) -> Self {
        Self { percent: 0.0, pixels }
    }

    pub fn pct(percent: f32) -> Self {
        Self { percent, pixels: 0.0 }
    }

    pub fn frac(fraction: f32) -> Self {
        Self { percent: fraction * 100.0, pixels: 0.0 }
    }

    pub fn to_pixels(&self, parent_size: f32) -> f32 {
        parent_size * (self.percent / 100.0) + self.pixels
    }
}

impl std::ops::Add for Length {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            percent: self.percent + rhs.percent,
            pixels: self.pixels + rhs.pixels,
        }
    }
}

impl From<f32> for Length {
    fn from(val: f32) -> Self {
        if val.abs() <= 1.0 {
            // Treat as fraction/percentage
            Self::frac(val)
        } else {
            // Treat as absolute pixels
            Self::px(val)
        }
    }
}

impl From<Length> for Position {
    fn from(length: Length) -> Self {
        Self::FromStart(length)
    }
}

impl From<f32> for Position {
    fn from(val: f32) -> Self {
        Self::FromStart(Length::from(val))
    }
}

pub fn px(val: f32) -> Length {
    Length::px(val)
}

pub fn pct(val: f32) -> Length {
    Length::pct(val)
}

pub fn frac(val: f32) -> Length {
    Length::frac(val)
}

pub fn from_start(l: impl Into<Length>) -> Position {
    Position::FromStart(l.into())
}

pub fn from_end(l: impl Into<Length>) -> Position {
    Position::FromEnd(l.into())
}

pub fn center_offset(l: impl Into<Length>) -> Position {
    Position::Center(l.into())
}

pub const CENTER: Position = Position::Center(Length::ZERO);

/// Declarative page layout (like HTML structure).
#[derive(Clone, Debug)]
pub struct Layout {
    pub items: Vec<LayoutItem>,
}

#[derive(Clone, Debug)]
pub struct LayoutItem {
    pub element: Element,
    pub x: Position,
    pub y: Position,
    pub width: Option<Length>,
    pub height: Option<Length>,
}

#[derive(Clone, Debug, Default)]
pub struct ButtonStyle {
    pub bg: Option<egui::Color32>,
    pub text: Option<egui::Color32>,
    pub hover_bg: Option<egui::Color32>,
    pub rounding: Option<f32>,
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
        scale: f32,
    },
    Button {
        label: String,
        id: String,
        style_tag: Option<String>,
        style: ButtonStyle,
        requires: Option<String>,
    },
}

pub trait ToLayoutItem {
    fn to_layout_item(self) -> LayoutItem;
}

impl Layout {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add(mut self, item: impl ToLayoutItem) -> Self {
        self.items.push(item.to_layout_item());
        self
    }

    pub fn text(
        self,
        content: impl Into<String>,
        x: impl Into<Position>,
        y: impl Into<Position>,
    ) -> Self {
        self.add(Text::new(content).x(x).y(y))
    }

    pub fn text_scaled(
        self,
        content: impl Into<String>,
        x: impl Into<Position>,
        y: impl Into<Position>,
        scale: f32,
    ) -> Self {
        self.add(Text::new(content).x(x).y(y).scale(scale))
    }

    pub fn checkbox(
        self,
        label: impl Into<String>,
        id: impl Into<String>,
        x: impl Into<Position>,
        y: impl Into<Position>,
    ) -> Self {
        self.add(Checkbox::new(label, id).x(x).y(y))
    }

    pub fn button(
        self,
        label: impl Into<String>,
        id: impl Into<String>,
        x: impl Into<Position>,
        y: impl Into<Position>,
    ) -> Self {
        self.add(Button::new(label, id).x(x).y(y))
    }

    pub fn button_requires(
        self,
        label: impl Into<String>,
        id: impl Into<String>,
        requires: impl Into<String>,
        x: impl Into<Position>,
        y: impl Into<Position>,
    ) -> Self {
        self.add(Button::new(label, id).x(x).y(y).requires(requires))
    }
}

/// Text builder.
#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub scale: f32,
    pub x: Position,
    pub y: Position,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            scale: 1.0,
            x: CENTER,
            y: CENTER,
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn x(mut self, x: impl Into<Position>) -> Self {
        self.x = x.into();
        self
    }

    pub fn y(mut self, y: impl Into<Position>) -> Self {
        self.y = y.into();
        self
    }
}

impl ToLayoutItem for Text {
    fn to_layout_item(self) -> LayoutItem {
        LayoutItem {
            element: Element::Text {
                content: self.content,
                scale: self.scale,
            },
            x: self.x,
            y: self.y,
            width: None,
            height: None,
        }
    }
}

/// Checkbox builder.
#[derive(Clone, Debug)]
pub struct Checkbox {
    pub label: String,
    pub id: String,
    pub scale: f32,
    pub x: Position,
    pub y: Position,
}

impl Checkbox {
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            scale: 1.0,
            x: CENTER,
            y: CENTER,
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn x(mut self, x: impl Into<Position>) -> Self {
        self.x = x.into();
        self
    }

    pub fn y(mut self, y: impl Into<Position>) -> Self {
        self.y = y.into();
        self
    }
}

impl ToLayoutItem for Checkbox {
    fn to_layout_item(self) -> LayoutItem {
        LayoutItem {
            element: Element::Checkbox {
                label: self.label,
                id: self.id,
                scale: self.scale,
            },
            x: self.x,
            y: self.y,
            width: None,
            height: None,
        }
    }
}

/// Button builder.
#[derive(Clone, Debug)]
pub struct Button {
    pub label: String,
    pub id: String,
    pub x: Position,
    pub y: Position,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub style_tag: Option<String>,
    pub requires: Option<String>,
}

impl Button {
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            x: CENTER,
            y: CENTER,
            width: None,
            height: None,
            style_tag: None,
            requires: None,
        }
    }

    pub fn x(mut self, x: impl Into<Position>) -> Self {
        self.x = x.into();
        self
    }

    pub fn y(mut self, y: impl Into<Position>) -> Self {
        self.y = y.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn style_tag(mut self, tag: impl Into<String>) -> Self {
        self.style_tag = Some(tag.into());
        self
    }

    pub fn requires(mut self, id: impl Into<String>) -> Self {
        self.requires = Some(id.into());
        self
    }
}

impl ToLayoutItem for Button {
    fn to_layout_item(self) -> LayoutItem {
        LayoutItem {
            element: Element::Button {
                label: self.label,
                id: self.id,
                style_tag: self.style_tag,
                style: ButtonStyle::default(),
                requires: self.requires,
            },
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}
