use std::sync::Arc;

use pollster::FutureExt;

use super::gpu::GpuDevice;

pub struct GraphicsContext<'a> {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'a>,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub _window: Arc<winit::window::Window>,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(window: Arc<winit::window::Window>, instance: &wgpu::Instance) -> Self {
        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = GraphicsContext::create_adapter(instance, &surface).block_on();
        let config = GraphicsContext::create_surface_config(size, &surface, &adapter);

        Self {
            size,
            surface,
            config,
            adapter,
            _window: window,
        }
    }

    pub fn configure(&self, gpu_device: &GpuDevice) {
        self.surface.configure(&gpu_device.device, &self.config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, gpu_device: &GpuDevice) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.configure(gpu_device);
        }
    }

    async fn create_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'a>,
    ) -> wgpu::Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .unwrap()
    }

    fn create_surface_config(
        size: winit::dpi::PhysicalSize<u32>,
        surface: &wgpu::Surface<'a>,
        adapter: &wgpu::Adapter,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
}
