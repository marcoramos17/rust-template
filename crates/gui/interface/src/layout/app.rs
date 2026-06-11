pub struct AppWindowSpec {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub bg_color: [f64; 4],
    pub window_mode: WindowMode,
}

pub fn app_window_spec() -> AppWindowSpec {
    AppWindowSpec {
        title: "My App".into(),
        width: 1280,
        height: 720,
        bg_color: [0.02, 0.14, 0.24, 1.0],
        window_mode: WindowMode::Borderless,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}

#[derive(Clone, Debug)]
pub enum UIElement {
    Text(String),
    Button { label: String, enabled: bool },
    Checkbox { label: String, checked: bool },
}

pub fn app_layout() -> Vec<UIElement> {
    vec![
        UIElement::Text("Welcome to My App".into()),
        UIElement::Checkbox { label: "Agree to terms".into(), checked: false },
        UIElement::Button { label: "Continue".into(), enabled: false },
    ]
}
