fn main() {
    if let Err(err) = rust_template_render::run() {
        eprintln!("{err}");
    }
}
