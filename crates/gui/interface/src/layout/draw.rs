use std::collections::HashMap;

use egui::{Context, RichText};

use super::spec::{Element, Layout, LayoutItem};

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

fn draw_item(
    ui: &mut egui::Ui,
    screen: egui::Rect,
    item: &LayoutItem,
    state: &mut LayoutState,
) {
    let width = screen.width() * item.width_pct;
    let height = screen.height() * item.height_pct;
    let center = egui::pos2(
        screen.left() + screen.width() * item.x_pct,
        screen.top() + screen.height() * item.y_pct,
    );
    let rect = egui::Rect::from_center_size(center, egui::vec2(width, height));

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
            ui.set_min_size(egui::vec2(width, height));

            match &item.element {
                Element::Text { content, scale } => {
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        ui.label(
                            RichText::new(content)
                                .size(ui.style().text_styles[&egui::TextStyle::Body].size * scale),
                        );
                    });
                }
                Element::Checkbox { label, id } => {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        let mut checked = *state.checkbox(id);
                        if ui.checkbox(&mut checked, label).changed() {
                            *state.checkbox(id) = checked;
                        }
                    });
                }
                Element::Button {
                    label,
                    id: _,
                    requires,
                } => {
                    let enabled = requires
                        .as_ref()
                        .map(|req| state.is_checked(req))
                        .unwrap_or(true);

                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        let button = egui::Button::new(label);
                        let response = ui.add_enabled(enabled, button);
                        let _ = response.clicked();
                    });
                }
            }
        });
}
