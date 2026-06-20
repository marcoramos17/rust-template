use rust_template_interface::layout::app::{app_window_spec, render_app_ui, WindowMode};
use rust_template_interface::styling::theme::apply_theme;
use rust_template_render::LayoutState;

fn main() {
    let spec_data = app_window_spec();

    let window_type = match spec_data.window_mode {
        WindowMode::Windowed => rust_template_render::WindowType::Windowed,
        WindowMode::Fullscreen => rust_template_render::WindowType::Fullscreen,
        WindowMode::Borderless => rust_template_render::WindowType::Borderless,
    };

    let spec = rust_template_render::WindowSpec {
        title: spec_data.title,
        width: spec_data.width,
        height: spec_data.height,
        window_type,
    };

    let layout_state = LayoutState::default();

    if let Err(err) = rust_template_render::run_with_ui(
        spec,
        layout_state,
        |ctx, state| render_app_ui(ctx, state),
        apply_theme,
    ) {
        eprintln!("{err}");
    }
}
