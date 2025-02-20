#![allow(unused)]

use anyhow::Context;
use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::pipelines;

mod error;

pub struct Renderer {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub window: Arc<winit::window::Window>,
    // -----------------------------------
    pub grid: pipelines::Grid,
}

impl Renderer {
    pub async fn new(window: Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let size = window.inner_size();
        let surface = instance
            .create_surface(window.clone())
            .map_err(error::RendererError::FailedToCreateSurface)?;
        let adapter = Self::create_adapter(&instance, &surface).await?;
        let config = Self::create_surface_config(&size, &surface, &adapter).await;
        let (device, queue) = Self::create_device_and_queue(&adapter).await?;
        surface.configure(&device, &config);

        let grid = pipelines::Grid::new(&device, &config);

        Ok(Self {
            size,
            surface,
            config,
            device,
            queue,
            window,
            grid,
        })
    }

    pub fn render(&mut self, dt: f32) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("compute_pas"),
                timestamp_writes: None,
            });

            self.grid.update(dt, &mut compute_pass);
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.grid.draw(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, size: Option<winit::dpi::PhysicalSize<u32>>) {
        if let Some(size) = size {
            if size.width > 0 && size.height > 0 {
                self.size = size;
                self.config.width = size.width;
                self.config.height = size.height;
                self.surface.configure(&self.device, &self.config);
            }
        }
    }

    async fn create_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'static>,
    ) -> anyhow::Result<wgpu::Adapter> {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .context(error::RendererError::FailedToCreateAdapter)
    }

    async fn create_surface_config(
        size: &winit::dpi::PhysicalSize<u32>,
        surface: &wgpu::Surface<'static>,
        adapter: &wgpu::Adapter,
    ) -> wgpu::SurfaceConfiguration {
        let surface_capabilities = surface.get_capabilities(adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    async fn create_device_and_queue(
        adapter: &wgpu::Adapter,
    ) -> anyhow::Result<(wgpu::Device, wgpu::Queue)> {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::VERTEX_WRITABLE_STORAGE,
                    ..Default::default()
                },
                None,
            )
            .await
            .map_err(|e| error::RendererError::FailedToCreateDeviceAndQueue(e).into())
    }
}
