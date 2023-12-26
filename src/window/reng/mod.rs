pub mod data;
mod resources;
pub mod utils;

pub use data::*;

use std::sync::*;

pub type CacheId = Arc<usize>;

#[derive(Debug)]
enum Command {
	CachedDraw { id: CacheId },
	UncachedDraw { count: u32 },
}

pub struct Renderer<UniformType: Copy + PartialEq, InstanceType> {
	resources: resources::RenderResources2D<UniformType, InstanceType>,
	render_data: data::RenderData,
	pub uniform: Option<UniformType>,

	sprites: Vec<InstanceType>,
	commands: Vec<Command>,
}

impl<UniformType: Copy + PartialEq, InstanceType> Renderer<UniformType, InstanceType> {
	const PRELOAD: usize = 25_000;

	const DEFAULT_CHUNK_SIZE: wgpu::BufferAddress =
		(Self::PRELOAD * std::mem::size_of::<InstanceType>()) as wgpu::BufferAddress;

	pub fn new(win: &winit::window::Window, sample_count: u32) -> Self {
		let resources =
			resources::RenderResources2D::<UniformType, InstanceType>::new(win, sample_count);

		let uniform_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Uniform"),
			size: std::mem::size_of::<UniformType>() as wgpu::BufferAddress,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let uniform_bg = resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: None,
				layout: &resources.uniform_bgl,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
						buffer: &uniform_buffer,
						offset: 0,
						size: None,
					}),
				}],
			});

		let instance_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Instance"),
			size: (Self::PRELOAD * std::mem::size_of::<InstanceType>()) as wgpu::BufferAddress,
			usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let instance_bg = resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: None,
				layout: &resources.instance_bgl,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
						buffer: &instance_buffer,
						offset: 0,
						size: None,
					}),
				}],
			});

		let def_image = image::ImageBuffer::from_pixel(1, 1, image::Rgba([255, 255, 255, 255]));

		let texture = resources.create_texture_from_image(&def_image);

		let sampler = resources.device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("nearest sampler"),
			address_mode_u: wgpu::AddressMode::MirrorRepeat,
			address_mode_v: wgpu::AddressMode::MirrorRepeat,
			address_mode_w: wgpu::AddressMode::MirrorRepeat,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});

		let texture_bg = resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: Some("default texture"),
				layout: &resources.texture_bgl,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: wgpu::BindingResource::TextureView(
							&texture.create_view(&wgpu::TextureViewDescriptor::default()),
						),
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: wgpu::BindingResource::Sampler(&sampler),
					},
				],
			});

		let render_data = data::RenderData {
			uniform_buffer,
			uniform_bg,
			instance_buffer,
			instance_bg,
			instance_cap: Self::PRELOAD,
			texture_bg,
			nearest_sampler: sampler,
			current_frame: None,
			clear_color: wgpu::Color::RED,
			cached_buffers: Default::default(),
			cached_count: 0,
		};

		Self {
			resources,
			render_data,
			uniform: None,

			sprites: vec![],
			commands: vec![],
		}
	}

	pub fn clear(&mut self, color: wgpu::Color) {
		self.sprites.clear();
		self.commands.clear();

		self.render_data.clear_color = color;
	}

	//Optional optimization
	pub fn reserve(&mut self, n: usize) {
		self.sprites.reserve(n);
	}

	pub fn queue(&mut self, instance: InstanceType) {
		//clip unseen instances
		self.sprites.push(instance);

		match self.commands.last_mut() {
			Some(Command::UncachedDraw { count }) => {
				*count += 1;
			}
			_ => {
				self.commands.push(Command::UncachedDraw { count: 1 });
			}
		}
	}

	pub fn set_texture(&mut self, texture: &wgpu::Texture) {
		self.render_data.texture_bg =
			self.resources
				.device
				.create_bind_group(&wgpu::BindGroupDescriptor {
					label: None,
					layout: &self.resources.texture_bgl,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::TextureView(
								&texture.create_view(&wgpu::TextureViewDescriptor::default()),
							),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::Sampler(
								&self.render_data.nearest_sampler,
							),
						},
					],
				});
	}

	pub fn flush(&mut self, uniform: UniformType) {
		self.store_uniform(uniform);
		self.store_instances();

		let ops = wgpu::Operations {
			load: wgpu::LoadOp::Clear(self.render_data.clear_color),
			store: wgpu::StoreOp::Store,
		};

		let view = &self.get_frame().texture.create_view(&Default::default());

		let mut encoder = self.resources.create_encoder();
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view,
				resolve_target: None,
				ops,
			})],
			..Default::default()
		});

		render_pass.set_pipeline(&self.resources.pipeline);
		render_pass.set_bind_group(0, &self.render_data.uniform_bg, &[]);
		render_pass.set_bind_group(2, &self.render_data.texture_bg, &[]);

		let mut i = 0;
		for command in self.commands.drain(..) {
			use Command::*;
			match command {
				CachedDraw { id } => {
					let (len, bg, _buffer) = &self.render_data.cached_buffers[&id];
					render_pass.set_bind_group(1, bg, &[]);
					render_pass.draw(0..5, 0..*len as u32);
				}
				UncachedDraw { count } => {
					render_pass.set_bind_group(1, &self.render_data.instance_bg, &[]);
					render_pass.draw(0..5, i..i + count);
					i += count;
				}
			}
		}

		std::mem::drop(render_pass);

		self.resources.queue.submit(Some(encoder.finish()));

		if let Some(frame) = self.render_data.current_frame.take() {
			frame.present();
		}
	}

	pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
		self.resources.resize(dims);
	}

	pub fn cache(&mut self, instances: &[InstanceType]) -> CacheId {
		use wgpu::util::*;
		let buffer = self
			.resources
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: None,
				contents: utils::to_char_slice(instances),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::STORAGE,
			});

		let bg = self
			.resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: None,
				layout: &self.resources.instance_bgl,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
						buffer: &buffer,
						offset: 0,
						size: None,
					}),
				}],
			});

		let id = CacheId::new(self.render_data.cached_count);
		self.render_data.cached_count += 1;

		self.render_data
			.cached_buffers
			.insert(id.clone(), (instances.len(), bg, buffer));

		id
	}

	pub fn queue_cached(&mut self, id: CacheId) {
		self.commands.push(Command::CachedDraw { id })
	}

	pub fn clean_cache(&mut self) {
		self.render_data
			.cached_buffers
			.retain(|k, _| Arc::strong_count(k) > 1);
	}

	pub fn create_texture_from_image(&self, image: &image::RgbaImage) -> wgpu::Texture {
		self.resources.create_texture_from_image(image)
	}

	fn store_uniform(&mut self, uniform: UniformType) {
		if self.uniform != Some(uniform) {
			self.uniform = Some(uniform);
			let unif_data = &[uniform];
			let unif_slice = utils::to_char_slice(unif_data);
			self.resources.queue.write_buffer(
				&self.render_data.uniform_buffer,
				0 as wgpu::BufferAddress,
				unif_slice,
			);
		}
	}

	fn store_instances(&mut self) {
		let len = self.sprites.len();
		let cap = self.sprites.capacity();

		if self.render_data.instance_cap < len {
			self.render_data.instance_cap = cap;

			self.render_data.instance_buffer =
				self.resources
					.device
					.create_buffer(&wgpu::BufferDescriptor {
						label: Some("Instance"),
						size: cap as wgpu::BufferAddress,
						usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
						mapped_at_creation: false,
					});

			self.render_data.instance_bg =
				self.resources
					.device
					.create_bind_group(&wgpu::BindGroupDescriptor {
						label: None,
						layout: &self.resources.instance_bgl,
						entries: &[wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
								buffer: &self.render_data.instance_buffer,
								offset: 0,
								size: None,
							}),
						}],
					});
		}

		let inst_slice = utils::to_char_slice(&self.sprites);

		self.resources.queue.write_buffer(
			&self.render_data.instance_buffer,
			0 as wgpu::BufferAddress,
			inst_slice,
		);
	}

	fn get_frame(&mut self) -> &wgpu::SurfaceTexture {
		self.render_data
			.current_frame
			.get_or_insert_with(|| self.resources.generate_frame())
	}
}
