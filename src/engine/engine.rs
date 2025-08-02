use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use anyhow::Context;

#[derive(Default)]
pub struct Engine {
    window: Option<Window>,
}

impl Engine {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Engine::default())
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new()
            .context("Failed to create event loop")
            .unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
