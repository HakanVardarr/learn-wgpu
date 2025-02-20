use std::time::Instant;

use super::engine::Engine;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};

#[derive(Default)]
pub struct Application {
    engine: Option<Engine>,
    last_frame_time: Option<Instant>,
}

impl Application {
    pub fn run() -> anyhow::Result<()> {
        let mut application = Self::default();
        let event_loop = EventLoop::new()?;

        event_loop.run_app(&mut application).map_err(|e| e.into())
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Game of Life")
                    .with_inner_size(winit::dpi::LogicalSize::new(800, 800)),
            )
            .expect("Failed to create windw!");

        self.engine = Some(Engine::new(std::sync::Arc::new(window)).unwrap());
        self.last_frame_time = Some(Instant::now());
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
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                engine.renderer.resize(Some(new_size));
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now
                    .duration_since(self.last_frame_time.unwrap())
                    .as_secs_f32();
                self.last_frame_time = Some(now);

                engine.request_redraw();
                match engine.renderer.render(dt) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        engine.renderer.resize(None);
                        // engine.resize(engine.graphics_context.size);
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
