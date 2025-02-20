use super::renderer::Renderer;
use pollster::FutureExt;

pub struct Engine {
    pub renderer: Renderer,
}

impl Engine {
    pub fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let renderer = Renderer::new(window).block_on()?;

        Ok(Self { renderer })
    }

    pub fn request_redraw(&self) {
        self.renderer.window.request_redraw();
    }
}
