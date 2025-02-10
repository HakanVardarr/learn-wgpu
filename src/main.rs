use learn_wgpu::run;
use pollster::FutureExt;

fn main() {
    env_logger::init();

    match run().block_on() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {e}")
        }
    }
}
