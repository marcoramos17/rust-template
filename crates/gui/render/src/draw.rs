use std::collections::HashMap;

use egui::{Context, RichText};

use super::spec::{Element, Layout, LayoutItem, Length, Position};

/// Interactive state keyed by element id (checkboxes, etc.).
#[derive(Clone, Debug, Default)]
pub struct LayoutState {
    pub checkboxes: HashMap<String, bool>,
}

impl LayoutState {
    pub fn checkbox(&mut self, id: &str) -> &mut bool {
        self.checkboxes.entry(id.to_owned()).or_insert(false)
    }

    pub fn is_checked(&self, id: &str) -> bool {
        self.checkboxes.get(id).copied().unwrap_or(false)
    }
}

pub fn draw_layout(ctx: &Context, layout: &Layout, state: &mut LayoutState) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(ctx.style().visuals.window_fill))
        .show(ctx, |ui| {
            let screen = ui.max_rect();
            for item in &layout.items {
                draw_item(ui, screen, item, state);
            }
        });
}

impl LayoutItem {
    pub fn resolve_width(&self, screen_width: f32) -> f32 {
        let default_val = match &self.element {
            Element::Text { .. } => Length::frac(0.9),
            Element::Checkbox { .. } => Length::frac(0.6),
            Element::Button { .. } => Length::frac(0.35),
        };
        self.width.unwrap_or(default_val).to_pixels(screen_width)
    }

    pub fn resolve_height(&self, screen_height: f32) -> f32 {
        let default_val = match &self.element {
            Element::Text { .. } => Length::frac(0.1),
            Element::Checkbox { .. } => Length::frac(0.08),
            Element::Button { .. } => Length::frac(0.08),
        };
        self.height.unwrap_or(default_val).to_pixels(screen_height)
    }
}

fn draw_item(
    ui: &mut egui::Ui,
    screen: egui::Rect,
    item: &LayoutItem,
    state: &mut LayoutState,
) {
    let width = item.resolve_width(screen.width());
    let height = item.resolve_height(screen.height());

    let x = match item.x {
        Position::FromStart(ref l) => screen.left() + l.to_pixels(screen.width()),
        Position::FromEnd(ref l) => screen.right() - l.to_pixels(screen.width()) - width,
        Position::Center(ref l) => screen.left() + (screen.width() - width) / 2.0 + l.to_pixels(screen.width()),
    };

    let y = match item.y {
        Position::FromStart(ref l) => screen.top() + l.to_pixels(screen.height()),
        Position::FromEnd(ref l) => screen.bottom() - l.to_pixels(screen.height()) - height,
        Position::Center(ref l) => screen.top() + (screen.height() - height) / 2.0 + l.to_pixels(screen.height()),
    };

    let rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, height));

    let id = egui::Id::new(match &item.element {
        Element::Text { content, .. } => content.as_str(),
        Element::Checkbox { id, .. } => id.as_str(),
        Element::Button { id, .. } => id.as_str(),
    });

    egui::Area::new(id)
        .fixed_pos(rect.left_top())
        .interactable(matches!(
            item.element,
            Element::Checkbox { .. } | Element::Button { .. }
        ))
        .show(ui.ctx(), |ui| {
            ui.allocate_ui(egui::vec2(width, height), |ui| {
                match &item.element {
                    Element::Text { content, scale } => {
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.label(
                                    RichText::new(content).size(
                                        ui.style().text_styles[&egui::TextStyle::Body].size
                                            * scale,
                                    ),
                                );
                            },
                        );
                    }
                    Element::Checkbox { label, id, scale } => {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            let orig_spacing = ui.spacing().icon_width;
                            ui.spacing_mut().icon_width = orig_spacing * scale;

                            let mut checked = *state.checkbox(id);
                            let rich_label = RichText::new(label).size(
                                ui.style().text_styles[&egui::TextStyle::Body].size * scale,
                            );
                            if ui.checkbox(&mut checked, rich_label).changed() {
                                *state.checkbox(id) = checked;
                            }

                            ui.spacing_mut().icon_width = orig_spacing;
                        });
                    }
                    Element::Button {
                        label,
                        id: _,
                        style_tag: _,
                        style,
                        requires,
                    } => {
                        let enabled = requires
                            .as_ref()
                            .map(|req| state.is_checked(req))
                            .unwrap_or(true);

                        if let Some(bg) = style.bg {
                            ui.style_mut().visuals.widgets.inactive.bg_fill = bg;
                        }
                        if let Some(text) = style.text {
                            ui.style_mut().visuals.widgets.inactive.fg_stroke.color = text;
                            ui.style_mut().visuals.widgets.hovered.fg_stroke.color = text;
                            ui.style_mut().visuals.widgets.active.fg_stroke.color = text;
                        }
                        if let Some(hover_bg) = style.hover_bg {
                            ui.style_mut().visuals.widgets.hovered.bg_fill = hover_bg;
                        }
                        if let Some(rounding) = style.rounding {
                            let rounding = egui::Rounding::same(rounding);
                            ui.style_mut().visuals.widgets.inactive.rounding = rounding;
                            ui.style_mut().visuals.widgets.hovered.rounding = rounding;
                            ui.style_mut().visuals.widgets.active.rounding = rounding;
                        }

                        ui.set_enabled(enabled);
                        let button = egui::Button::new(label);
                        let response = ui.add_sized(
                            egui::vec2(width, height),
                            button,
                        );
                        let _ = response.clicked();
                    }
                }
            });
        });
}
