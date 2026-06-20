use egui::{Color32, FontFamily, FontId, TextStyle, Visuals};
use serde::Deserialize;

const THEME_TOML: &str = include_str!("theme.toml");

#[derive(Debug, Deserialize)]
struct ThemeFile {
    colors: Colors,
    fonts: Fonts,
    widgets: Widgets,
    #[serde(default)]
    overrides: std::collections::HashMap<String, WidgetStyleOverride>,
}

#[derive(Debug, Deserialize)]
struct Colors {
    background: String,
    text: String,
    accent: String,
    panel: String,
}

#[derive(Debug, Deserialize)]
struct Fonts {
    family: String,
    size: f32,
    heading_scale: f32,
}

#[derive(Debug, Deserialize)]
struct Widgets {
    button: ButtonStyle,
    checkbox: CheckboxStyle,
}

#[derive(Debug, Deserialize)]
struct ButtonStyle {
    bg: String,
    text: String,
    hover_bg: String,
    rounding: f32,
}

#[derive(Debug, Deserialize)]
struct CheckboxStyle {
    bg: String,
    stroke: String,
}

#[derive(Debug, Deserialize, Clone)]
struct WidgetStyleOverride {
    button: Option<ButtonStyleOverride>,
}

#[derive(Debug, Deserialize, Clone)]
struct ButtonStyleOverride {
    bg: Option<String>,
    text: Option<String>,
    hover_bg: Option<String>,
    rounding: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ButtonStyleResolved {
    pub bg: Option<Color32>,
    pub text: Option<Color32>,
    pub hover_bg: Option<Color32>,
    pub rounding: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color32,
    pub text: Color32,
    pub accent: Color32,
    pub panel: Color32,
    pub font_family: FontFamily,
    pub font_size: f32,
    pub heading_scale: f32,
    pub button_bg: Color32,
    pub button_text: Color32,
    pub button_hover_bg: Color32,
    pub button_rounding: f32,
    pub checkbox_bg: Color32,
    pub checkbox_stroke: Color32,
    pub overrides: std::collections::HashMap<String, ButtonStyleResolved>,
}

impl Theme {
    pub fn load() -> Self {
        let file: ThemeFile = toml::from_str(THEME_TOML)
            .unwrap_or_else(|err| panic!("invalid theme.toml: {err}"));

        let mut overrides = std::collections::HashMap::new();
        for (tag, ov) in file.overrides {
            if let Some(btn) = ov.button {
                overrides.insert(
                    tag,
                    ButtonStyleResolved {
                        bg: btn.bg.as_deref().map(parse_color),
                        text: btn.text.as_deref().map(parse_color),
                        hover_bg: btn.hover_bg.as_deref().map(parse_color),
                        rounding: btn.rounding,
                    },
                );
            }
        }

        Self {
            background: parse_color(&file.colors.background),
            text: parse_color(&file.colors.text),
            accent: parse_color(&file.colors.accent),
            panel: parse_color(&file.colors.panel),
            font_family: parse_font_family(&file.fonts.family),
            font_size: file.fonts.size,
            heading_scale: file.fonts.heading_scale,
            button_bg: parse_color(&file.widgets.button.bg),
            button_text: parse_color(&file.widgets.button.text),
            button_hover_bg: parse_color(&file.widgets.button.hover_bg),
            button_rounding: file.widgets.button.rounding,
            checkbox_bg: parse_color(&file.widgets.checkbox.bg),
            checkbox_stroke: parse_color(&file.widgets.checkbox.stroke),
            overrides,
        }
    }

    pub fn background_rgb(&self) -> [f64; 4] {
        [
            self.background.r() as f64 / 255.0,
            self.background.g() as f64 / 255.0,
            self.background.b() as f64 / 255.0,
            self.background.a() as f64 / 255.0,
        ]
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = if ctx.style().visuals.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        visuals.window_fill = self.background;
        visuals.panel_fill = self.panel;
        visuals.override_text_color = Some(self.text);
        visuals.widgets.noninteractive.bg_fill = self.panel;
        visuals.widgets.inactive.bg_fill = self.button_bg;
        visuals.widgets.inactive.fg_stroke.color = self.button_text;
        visuals.widgets.hovered.bg_fill = self.button_hover_bg;
        visuals.widgets.hovered.fg_stroke.color = self.button_text;
        visuals.widgets.active.bg_fill = self.accent;
        visuals.widgets.active.fg_stroke.color = self.button_text;
        visuals.selection.bg_fill = self.accent;
        visuals.widgets.inactive.rounding = egui::Rounding::same(self.button_rounding);
        visuals.widgets.hovered.rounding = egui::Rounding::same(self.button_rounding);
        visuals.widgets.active.rounding = egui::Rounding::same(self.button_rounding);
        visuals.widgets.inactive.bg_stroke.color = self.checkbox_stroke;
        visuals.widgets.inactive.bg_stroke.width = 1.0;

        ctx.set_visuals(visuals);

        let body = FontId::new(self.font_size, self.font_family.clone());
        let heading = FontId::new(self.font_size * self.heading_scale, self.font_family.clone());

        ctx.style_mut(|style| {
            style.text_styles.insert(TextStyle::Button, body.clone());
            style.text_styles.insert(TextStyle::Body, body);
            style.text_styles.insert(TextStyle::Heading, heading);
        });
    }
}

fn parse_color(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Color32::from_rgb(r, g, b)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Color32::from_rgba_unmultiplied(r, g, b, a)
        }
        _ => Color32::WHITE,
    }
}

fn parse_font_family(name: &str) -> FontFamily {
    match name.to_lowercase().as_str() {
        "monospace" | "mono" => FontFamily::Monospace,
        "proportional" | "sans" | "inter" => FontFamily::Proportional,
        other => FontFamily::Name(other.into()),
    }
}

pub fn apply_theme(ctx: &egui::Context) {
    Theme::load().apply(ctx);
}

pub fn apply_layout_styles(layout: &mut rust_template_render::Layout) {
    let theme = Theme::load();
    for item in &mut layout.items {
        if let rust_template_render::Element::Button { id, style_tag, style, .. } = &mut item.element
        {
            // Base styles for all buttons (like `button { ... }` in CSS).
            style.bg = Some(theme.button_bg);
            style.text = Some(theme.button_text);
            style.hover_bg = Some(theme.button_hover_bg);
            style.rounding = Some(theme.button_rounding);

            // Tag/id overrides (like `button.success { ... }` or `#continue { ... }`).
            let lookup_keys = [style_tag.as_deref(), Some(id.as_str())];
            for key in lookup_keys.into_iter().flatten() {
                if let Some(resolved) = theme.overrides.get(key) {
                    if let Some(bg) = resolved.bg {
                        style.bg = Some(bg);
                    }
                    if let Some(text) = resolved.text {
                        style.text = Some(text);
                    }
                    if let Some(hover_bg) = resolved.hover_bg {
                        style.hover_bg = Some(hover_bg);
                    }
                    if let Some(rounding) = resolved.rounding {
                        style.rounding = Some(rounding);
                    }
                    break;
                }
            }
        }
    }
}
