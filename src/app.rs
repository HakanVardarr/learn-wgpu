use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowAttributes;

use crate::engine::Engine;

#[derive(Default)]
pub struct App<'a> {
    engine: Option<Engine<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Hello World!"))
            .unwrap();
        self.engine = Some(Engine::new(std::sync::Arc::new(window)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let engine = self.engine.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                engine.resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                engine.request_redraw();
                match engine.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        engine.resize(engine.graphics_context.size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                        event_loop.exit();
                    }
                    Err(wgpu::SurfaceError::Timeout) => {}
                }
            }
            _ => {}
        }
    }
}
