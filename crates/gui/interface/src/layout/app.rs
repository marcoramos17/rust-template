use egui::Context;
use rust_template_render::{
    draw_layout, from_end, pct, Button, Checkbox, Layout, LayoutState, Text, CENTER,
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
            Text::new("Welcome to My App")
                .scale(1.5)
                .x(CENTER)
                .y(pct(15.0)),
        )
        .add(
            Checkbox::new("Agree to terms", "agree")
                .scale(1.2)
                .x(CENTER)
                .y(pct(45.0)),
        )
        .add(
            Button::new("Continue", "continue")
                //.x(px(150.0))
                .y(from_end(pct(20.0)))
                .width(pct(30.0))
                .height(pct(10.0)) // fixed f32 conversion logic from integer
                //.style_tag("success")
                .requires("agree"),
        )
}

pub fn render_app_ui(ctx: &Context, state: &mut LayoutState) {
    let mut layout = app_layout();
    apply_layout_styles(&mut layout);
    draw_layout(ctx, &layout, state);
}
