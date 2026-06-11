fn main() {
    // Simple example entrypoint for the render crate
    let spec = rust_template_render::WindowSpec::default();
    if let Err(err) = rust_template_render::run_with_spec(spec) {
        eprintln!("{err}");
    }
}
