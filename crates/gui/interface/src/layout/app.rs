use egui::Context;
use rust_template_render::{
    draw_layout, pct, px, Button, Checkbox, Layout, LayoutState, Text, CENTER,
};
use crate::styling::theme::apply_layout_styles;

pub struct AppWindowSpec {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub window_mode: WindowMode,
}

pub fn app_window_spec() -> AppWindowSpec {
    AppWindowSpec {
        title: "My App".into(),
        width: 1280,
        height: 720,
        window_mode: WindowMode::Windowed,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}

pub fn app_layout() -> Layout {
    Layout::new()
        .add(
            Text::new("Hello, World!")
                .scale(3.0)
                .x(CENTER)
                .y(CENTER - pct(1.0)),
        )
        .add(
            Checkbox::new("Agree to terms", "agree")
                .scale(1.2)
                .x(CENTER)
                .y(px(30)),
        )
        .add(
            Button::new("Continue", "continue")
                .x(pct(100) - pct(1))
                .y(CENTER + pct(5))
                .width(pct(10))
                .height(pct(2) + px(20))
                .requires("agree"),
        )
        .add(
            Button::new("Cancel", "cancel")
                .x(pct(100) - pct(1))
                .y(CENTER - pct(5))
                .width(pct(10))
                .height(pct(2) + px(20)),
        )
        .add(
            Text::new("This is a test app GUI")
                .scale(1.0)
                .x(CENTER)
                .y(CENTER - pct(1.0)),
        )
}

pub fn render_app_ui(ctx: &Context, state: &mut LayoutState) {
    let mut layout = app_layout();
    apply_layout_styles(&mut layout);
    draw_layout(ctx, &layout, state);
}
