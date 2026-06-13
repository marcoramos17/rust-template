use egui::Context;

use super::draw::{draw_layout, LayoutState};
use super::spec::Layout;

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

/// Define what appears on the main window. Positions are fractions of the window
/// (0.0–1.0) so layout scales when the window is resized.
///
/// Example:
/// ```ignore
/// Layout::new()
///     .text_scaled("Welcome", 0.5, 0.15, 1.5)   // centered near top
///     .checkbox("Agree to terms", "agree", 0.5, 0.45)
///     .button_requires("Continue", "continue", "agree", 0.5, 0.75)
/// ```
pub fn app_layout() -> Layout {
    Layout::new()
        .text_scaled("Welcome to My App", 0.5, 0.15, 1.5)
        .checkbox("Agree to terms", "agree", 0.5, 0.45)
        .button_requires("Continue", "continue", "agree", 0.5, 0.75)
}

pub fn render_app_ui(ctx: &Context, state: &mut LayoutState) {
    draw_layout(ctx, &app_layout(), state);
}
