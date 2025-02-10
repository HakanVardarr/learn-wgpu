use app::App;
use winit::event_loop::EventLoop;

mod app;
mod engine;

pub async fn run() -> anyhow::Result<()> {
    let mut app = App::default();
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app).map_err(|e| e.into())
}
