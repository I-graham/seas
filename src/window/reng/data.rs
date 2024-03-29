use std::sync::Arc;

pub struct RenderData {
	pub uniform_buffer: wgpu::Buffer,
	pub uniform_bg: wgpu::BindGroup,
	pub instance_buffer: wgpu::Buffer,
	pub instance_bg: wgpu::BindGroup,
	pub instance_cap: usize,
	pub texture_bg: wgpu::BindGroup,
	pub nearest_sampler: wgpu::Sampler,
	pub current_frame: Option<wgpu::SurfaceTexture>,
	pub clear_color: wgpu::Color,
	pub cached_buffers: fnv::FnvHashMap<Arc<usize>, (usize, wgpu::BindGroup, wgpu::Buffer)>,
	pub cached_count: usize,
}
