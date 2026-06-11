use std::error::Error;

mod util {
    pub mod fill;
    pub mod tracing;
}

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
    pub bg_color: [f64; 4],
}

impl Default for WindowSpec {
    fn default() -> Self {
        Self {
            title: "rust_template".into(),
            width: 1280,
            height: 720,
            window_type: WindowType::Windowed,
            bg_color: [0.05, 0.15, 0.30, 1.0],
        }
    }
}

#[derive(Default)]
struct App {
    window: Option<Box<dyn Window>>,
    renderer: Option<Renderer>,
    spec: Option<WindowSpec>,
}

impl ApplicationHandler for App {
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
                // Note: initial size is requested via WindowAttributes min surface size.
                // apply window type choices after creation where possible
                match spec.window_type {
                    WindowType::Windowed => {
                        let _ = window.set_decorations(true);
                    }
                    WindowType::Fullscreen => {
                        let _ = window.set_fullscreen(Some(winit::monitor::Fullscreen::Borderless(None)));
                    }
                    WindowType::Borderless => {
                        let _ = window.set_decorations(false);
                    }
                }
                let renderer = match pollster::block_on(Renderer::new(window.as_ref(), spec.bg_color)) {
                    Ok(renderer) => Some(renderer),
                    Err(err) => {
                        tracing::error!(%err, "failed to initialize renderer");
                        event_loop.exit();
                        return;
                    }
                };

                self.renderer = renderer;
                Some(window)
            }
            Err(err) => {
                tracing::error!(%err, "error creating window");
                event_loop.exit();
                return;
            }
        };
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        tracing::info!(?event, "window event");
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Close was requested; stopping");
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(window) = &self.window {
                    let new_size = window.outer_size();
                    if let Some(renderer) = &mut self.renderer {
                        renderer.resize(new_size);
                    }
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    if let Some(window) = &self.window {
                        window.pre_present_notify();
                    }
                    match renderer.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => {
                            if let Some(window) = &self.window {
                                renderer.resize(window.outer_size());
                            }
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

pub fn run_with_spec(spec: WindowSpec) -> Result<(), Box<dyn Error>> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    init();

    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    app.spec = Some(spec);
    let app_box = Box::new(app);
    event_loop.run_app(app_box)?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    run_with_spec(WindowSpec::default())
}
