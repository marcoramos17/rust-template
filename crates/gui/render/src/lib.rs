use std::error::Error;

mod util {
    pub mod egui_winit;
    pub mod fill;
    pub mod tracing;
}

use util::egui_winit::State as EguiWinitState;
use util::fill::Renderer;
use util::tracing::init;
use wgpu::SurfaceError;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    dpi::PhysicalSize,
    window::{Window, WindowAttributes, WindowId},
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesWeb;

#[derive(Clone, Copy, Debug)]
pub enum WindowType {
    Windowed,
    Fullscreen,
    Borderless,
}

impl Default for WindowType {
    fn default() -> Self {
        WindowType::Windowed
    }
}

#[derive(Clone, Debug)]
pub struct WindowSpec {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub window_type: WindowType,
}

impl Default for WindowSpec {
    fn default() -> Self {
        Self {
            title: "rust_template".into(),
            width: 1280,
            height: 720,
            window_type: WindowType::Windowed,
        }
    }
}

struct App<S, F, T> {
    window: Option<Box<dyn Window>>,
    renderer: Option<Renderer>,
    egui_winit: Option<EguiWinitState>,
    spec: Option<WindowSpec>,
    state: S,
    ui: F,
    theme: T,
}

impl<S, F, T> ApplicationHandler for App<S, F, T>
where
    S: 'static,
    F: FnMut(&egui::Context, &mut S) + 'static,
    T: Fn(&egui::Context) + 'static,
{
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let spec = match &self.spec {
            Some(s) => s.clone(),
            None => WindowSpec::default(),
        };

        #[cfg(not(target_arch = "wasm32"))]
        let window_attributes = WindowAttributes::default()
            .with_title(spec.title.clone())
            .with_min_surface_size(PhysicalSize::new(spec.width, spec.height));

        #[cfg(target_arch = "wasm32")]
        let window_attributes = WindowAttributes::default()
            .with_platform_attributes(Box::new(WindowAttributesWeb::default().with_append(true)));

        self.window = match event_loop.create_window(window_attributes) {
            Ok(window) => {
                match spec.window_type {
                    WindowType::Windowed => {
                        let _ = window.set_decorations(true);
                    }
                    WindowType::Fullscreen => {
                        let _ = window
                            .set_fullscreen(Some(winit::monitor::Fullscreen::Borderless(None)));
                    }
                    WindowType::Borderless => {
                        let _ = window.set_decorations(false);
                    }
                }

                let renderer = match pollster::block_on(Renderer::new(window.as_ref())) {
                    Ok(renderer) => Some(renderer),
                    Err(err) => {
                        tracing::error!(%err, "failed to initialize renderer");
                        event_loop.exit();
                        return;
                    }
                };

                let egui_ctx = renderer.as_ref().unwrap().egui_ctx().clone();
                (self.theme)(&egui_ctx);

                self.egui_winit = Some(EguiWinitState::new(egui_ctx));
                self.renderer = renderer;
                window.request_redraw();
                Some(window)
            }
            Err(err) => {
                tracing::error!(%err, "error creating window");
                event_loop.exit();
                return;
            }
        };
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.window {
            Some(w) => w,
            None => return,
        };

        if let Some(egui_winit) = &mut self.egui_winit {
            let response = egui_winit.on_window_event(window.as_ref(), &event);
            if response.repaint {
                window.request_redraw();
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                let new_size = window.surface_size();
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size);
                }
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                window.pre_present_notify();

                let raw_input = self
                    .egui_winit
                    .as_mut()
                    .map(|state| state.take_input(window.as_ref()))
                    .unwrap_or_default();

                if let Some(renderer) = &mut self.renderer {
                    match renderer.render_with_input(
                        window.as_ref(),
                        raw_input,
                        |ctx| (self.ui)(ctx, &mut self.state),
                    ) {
                        Ok(full_output) => {
                            if let Some(egui_winit) = &mut self.egui_winit {
                                egui_winit.handle_platform_output(
                                    window.as_ref(),
                                    full_output.platform_output,
                                );
                            }
                        }
                        Err(SurfaceError::Lost) => {
                            renderer.resize(window.surface_size());
                            window.request_redraw();
                        }
                        Err(SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(err) => tracing::error!(?err, "render failed"),
                    }
                }
            }
            _ => {}
        }
    }
}

/// Run the GUI with window configuration, theme setup, and a UI builder.
pub fn run_with_ui<S, F, T>(
    spec: WindowSpec,
    state: S,
    ui: F,
    theme: T,
) -> Result<(), Box<dyn Error>>
where
    S: 'static,
    F: FnMut(&egui::Context, &mut S) + 'static,
    T: Fn(&egui::Context) + 'static,
{
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    init();

    let event_loop = EventLoop::new()?;
    let app = App {
        window: None,
        renderer: None,
        egui_winit: None,
        spec: Some(spec),
        state,
        ui,
        theme,
    };
    event_loop.run_app(Box::new(app))?;
    Ok(())
}

/// Run with default window settings and no UI (blank window).
pub fn run() -> Result<(), Box<dyn Error>> {
    run_with_ui(
        WindowSpec::default(),
        (),
        |_ctx, _state| {},
        |_ctx| {},
    )
}

/// Backward-compatible entry for callers that only supply a window spec.
pub fn run_with_spec(spec: WindowSpec) -> Result<(), Box<dyn Error>> {
    run_with_ui(spec, (), |_ctx, _state| {}, |_ctx| {})
}
