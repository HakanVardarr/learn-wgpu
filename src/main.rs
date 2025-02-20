use wgpu_engine::core::application::Application;

fn main() {
    env_logger::init();

    match Application::run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ERROR]: {e}");
        }
    }
}
