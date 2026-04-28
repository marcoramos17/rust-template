use std::error::Error;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default, Debug)]
struct App {
    window: Option<Box<dyn Window>>,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("FerrioSynth Window");

        match event_loop.create_window(window_attributes) {
            Ok(window) => self.window = Some(window),
            Err(err) => {
                eprintln!("error creating window: {err}");
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window closed!");
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(_) => {
                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                let window = match self.window.as_ref() {
                    Some(window) => window,
                    None => return,
                };

                // Notify that you're about to draw
                window.pre_present_notify();
                // Your rendering code goes here
            }
            _ => {}
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    event_loop.run_app(App::default())?;
    Ok(())
}
