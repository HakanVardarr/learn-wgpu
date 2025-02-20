use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Failed to create surface: ({0})")]
    FailedToCreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("Failed to create adapter")]
    FailedToCreateAdapter,
    #[error("Failde to create device and queue: ({0})")]
    FailedToCreateDeviceAndQueue(#[from] wgpu::RequestDeviceError),
}
