fn main() {
    // Example: build a window spec from the interface crate and run the renderer
    let spec_data = rust_template_interface::layout::app::app_window_spec();

    let window_type = match spec_data.window_mode {
        rust_template_interface::layout::app::WindowMode::Windowed => rust_template_render::WindowType::Windowed,
        rust_template_interface::layout::app::WindowMode::Fullscreen => rust_template_render::WindowType::Fullscreen,
        rust_template_interface::layout::app::WindowMode::Borderless => rust_template_render::WindowType::Borderless,
    };

    let spec = rust_template_render::WindowSpec {
        title: spec_data.title,
        width: spec_data.width,
        height: spec_data.height,
        window_type,
        bg_color: spec_data.bg_color,
    };

    if let Err(err) = rust_template_render::run_with_spec(spec) {
        eprintln!("{err}");
    }
}
