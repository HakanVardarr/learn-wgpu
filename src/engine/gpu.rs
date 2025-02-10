use pollster::FutureExt;

use super::context::GraphicsContext;

pub struct GpuDevice {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuDevice {
    pub fn new(graphics_context: &GraphicsContext) -> Self {
        let (device, queue) = GpuDevice::create_device(&graphics_context.adapter).block_on();

        Self { device, queue }
    }

    async fn create_device<'a>(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap()
    }
}
