//! Minimal egui ↔ winit 0.31 bridge.
//!
//! `egui-winit` on crates.io targets winit 0.30. This module adapts winit 0.31
//! events (e.g. `SurfaceResized`, `PointerMoved`, `PointerButton`) until an
//! official release supports the new API. Swap it out when `egui-winit` catches up.

use egui::{Event, Key, PointerButton, Pos2, RawInput, Rect, Vec2};
use web_time::Instant;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    keyboard::NamedKey,
    window::Window,
};

/// Whether egui consumed an event and whether a repaint is needed.
#[derive(Clone, Copy, Debug, Default)]
pub struct EventResponse {
    /// Whether egui consumed the event (useful if you also handle game input).
    pub consumed: bool,
    pub repaint: bool,
}

/// Translates winit window events into egui [`RawInput`].
pub struct State {
    egui_ctx: egui::Context,
    start_time: Instant,
    input: RawInput,
    pointer_pos_points: Option<Pos2>,
    any_pointer_down: bool,
}

impl State {
    pub fn new(egui_ctx: egui::Context) -> Self {
        Self {
            egui_ctx,
            start_time: Instant::now(),
            input: RawInput {
                focused: false,
                ..Default::default()
            },
            pointer_pos_points: None,
            any_pointer_down: false,
        }
    }

    pub fn egui_ctx(&self) -> &egui::Context {
        &self.egui_ctx
    }

    pub fn on_window_event(&mut self, window: &dyn Window, event: &WindowEvent) -> EventResponse {
        match event {
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.input
                    .viewports
                    .entry(egui::ViewportId::ROOT)
                    .or_default()
                    .native_pixels_per_point = Some(*scale_factor as f32);
                EventResponse {
                    repaint: true,
                    consumed: false,
                }
            }
            WindowEvent::PointerButton {
                state,
                position,
                button,
                primary,
                ..
            } => {
                if *primary {
                    self.set_pointer_pos(window, *position);
                    if let Some(mouse) = button.clone().mouse_button() {
                        self.on_mouse_button(*state, mouse);
                    }
                }
                EventResponse {
                    repaint: true,
                    consumed: self.egui_ctx.wants_pointer_input(),
                }
            }
            WindowEvent::PointerMoved {
                position, primary, ..
            } => {
                if *primary {
                    self.set_pointer_pos(window, *position);
                }
                EventResponse {
                    repaint: true,
                    consumed: self.egui_ctx.is_using_pointer(),
                }
            }
            WindowEvent::PointerLeft { primary, .. } => {
                if *primary {
                    self.pointer_pos_points = None;
                    self.input.events.push(Event::PointerGone);
                }
                EventResponse {
                    repaint: true,
                    consumed: false,
                }
            }
            WindowEvent::PointerEntered {
                position, primary, ..
            } => {
                if *primary {
                    self.set_pointer_pos(window, *position);
                }
                EventResponse {
                    repaint: true,
                    consumed: false,
                }
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                self.on_mouse_wheel(window, *delta, *phase);
                EventResponse {
                    repaint: true,
                    consumed: self.egui_ctx.wants_pointer_input(),
                }
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic,
                ..
            } => {
                if *is_synthetic && event.state == ElementState::Pressed {
                    return EventResponse {
                        repaint: true,
                        consumed: false,
                    };
                }

                self.on_keyboard_input(event);
                let wants_keyboard = self.egui_ctx.wants_keyboard_input();
                let tab = event.logical_key
                    == winit::keyboard::Key::Named(NamedKey::Tab);
                EventResponse {
                    repaint: true,
                    consumed: wants_keyboard || tab,
                }
            }
            WindowEvent::Focused(focused) => {
                self.input.focused = *focused;
                self.input.events.push(Event::WindowFocused(*focused));
                EventResponse {
                    repaint: true,
                    consumed: false,
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                let state = modifiers.state();
                self.input.modifiers.alt = state.alt_key();
                self.input.modifiers.ctrl = state.control_key();
                self.input.modifiers.shift = state.shift_key();
                self.input.modifiers.mac_cmd = cfg!(target_os = "macos") && state.meta_key();
                self.input.modifiers.command = if cfg!(target_os = "macos") {
                    state.meta_key()
                } else {
                    state.control_key()
                };
                EventResponse {
                    repaint: true,
                    consumed: false,
                }
            }
            WindowEvent::RedrawRequested
            | WindowEvent::SurfaceResized(_)
            | WindowEvent::CloseRequested
            | WindowEvent::Occluded(_) => EventResponse {
                repaint: true,
                consumed: false,
            },
            _ => EventResponse::default(),
        }
    }

    pub fn take_input(&mut self, window: &dyn Window) -> RawInput {
        self.input.time = Some(self.start_time.elapsed().as_secs_f64());

        let pixels_per_point = window.scale_factor() as f32 * self.egui_ctx.zoom_factor();
        let size = window.surface_size();
        let size_points = Vec2::new(
            size.width as f32 / pixels_per_point,
            size.height as f32 / pixels_per_point,
        );

        self.input.screen_rect = (size_points.x > 0.0 && size_points.y > 0.0)
            .then(|| Rect::from_min_size(Pos2::ZERO, size_points));

        self.input.viewport_id = egui::ViewportId::ROOT;
        self.input
            .viewports
            .entry(egui::ViewportId::ROOT)
            .or_default()
            .native_pixels_per_point = Some(pixels_per_point);

        self.input.take()
    }

    pub fn handle_platform_output(&mut self, _window: &dyn Window, output: egui::PlatformOutput) {
        let _ = output.cursor_icon;
        let _ = output.ime;
    }

    fn pixels_per_point(&self, window: &dyn Window) -> f32 {
        window.scale_factor() as f32 * self.egui_ctx.zoom_factor()
    }

    fn set_pointer_pos(&mut self, window: &dyn Window, pos: PhysicalPosition<f64>) {
        let ppp = self.pixels_per_point(window);
        let pos_points = Pos2::new(pos.x as f32 / ppp, pos.y as f32 / ppp);
        self.pointer_pos_points = Some(pos_points);
        self.input.events.push(Event::PointerMoved(pos_points));
    }

    fn on_mouse_button(&mut self, state: ElementState, button: MouseButton) {
        if let (Some(pos), Some(button)) = (
            self.pointer_pos_points,
            translate_mouse_button(button),
        ) {
            let pressed = state == ElementState::Pressed;
            self.input.events.push(Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers: self.input.modifiers,
            });
            self.any_pointer_down = pressed;
        }
    }

    fn on_mouse_wheel(
        &mut self,
        window: &dyn Window,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    ) {
        let ppp = self.pixels_per_point(window);
        let (unit, delta) = match delta {
            MouseScrollDelta::LineDelta(x, y) => (egui::MouseWheelUnit::Line, Vec2::new(x, y)),
            MouseScrollDelta::PixelDelta(pos) => (
                egui::MouseWheelUnit::Point,
                Vec2::new(pos.x as f32, pos.y as f32) / ppp,
            ),
        };
        let _ = phase;
        self.input.events.push(Event::MouseWheel {
            unit,
            delta,
            modifiers: self.input.modifiers,
        });
    }

    fn on_keyboard_input(&mut self, event: &winit::event::KeyEvent) {
        let pressed = event.state == ElementState::Pressed;

        if let Some(key) = key_from_winit(&event.logical_key) {
            self.input.events.push(Event::Key {
                key,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers: self.input.modifiers,
            });
        }

        if pressed {
            if let Some(text) = event
                .text
                .as_ref()
                .map(|t| t.as_str())
                .or_else(|| event.logical_key.to_text())
            {
                if !text.is_empty() && text.chars().all(is_printable_char) {
                    let is_cmd = self.input.modifiers.ctrl
                        || self.input.modifiers.command
                        || self.input.modifiers.mac_cmd;
                    if !is_cmd {
                        self.input.events.push(Event::Text(text.to_owned()));
                    }
                }
            }
        }
    }
}

fn translate_mouse_button(button: MouseButton) -> Option<PointerButton> {
    match button {
        MouseButton::Left => Some(PointerButton::Primary),
        MouseButton::Right => Some(PointerButton::Secondary),
        MouseButton::Middle => Some(PointerButton::Middle),
        MouseButton::Back => Some(PointerButton::Extra1),
        MouseButton::Forward => Some(PointerButton::Extra2),
        _ => None,
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';
    !is_in_private_use_area && !chr.is_ascii_control()
}

fn key_from_winit(key: &winit::keyboard::Key) -> Option<Key> {
    match key {
        winit::keyboard::Key::Named(named) => Some(match named {
            NamedKey::ArrowDown => Key::ArrowDown,
            NamedKey::ArrowLeft => Key::ArrowLeft,
            NamedKey::ArrowRight => Key::ArrowRight,
            NamedKey::ArrowUp => Key::ArrowUp,
            NamedKey::Escape => Key::Escape,
            NamedKey::Tab => Key::Tab,
            NamedKey::Backspace => Key::Backspace,
            NamedKey::Enter => Key::Enter,
            NamedKey::Insert => Key::Insert,
            NamedKey::Delete => Key::Delete,
            NamedKey::Home => Key::Home,
            NamedKey::End => Key::End,
            NamedKey::PageUp => Key::PageUp,
            NamedKey::PageDown => Key::PageDown,
            _ => return None,
        }),
        winit::keyboard::Key::Character(_) => None,
        _ => None,
    }
}
