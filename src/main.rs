mod engine;

use engine::Engine;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let engine = Engine::new(&event_loop);
}
