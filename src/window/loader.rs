use super::{glsl::*, reng::*, types::*};

pub fn load_textures<T: TextureType>() -> (image::RgbaImage, TextureMap) {
	let mut map = TextureMap::default();

	let list = T::list();

	let mut rgba_images = list
		.iter()
		.map(|text| {
			let file_name = format!("assets/{}.png", text.name());
			image::open(file_name)
				.expect("Unable to open image texture asset.")
				.into_rgba8()
		})
		.collect::<Vec<_>>();

	let img_size = |img: &image::RgbaImage| (img.height() * img.width()) as i32;

	let mut sorted_iter = list.iter().enumerate().collect::<Vec<_>>();
	sorted_iter.sort_by_key(|(index, _text)| -img_size(&rgba_images[*index]));

	rgba_images.sort_by_key(|e| -img_size(e));

	let spritesheet = utils::create_spritesheet(rgba_images);

	let image_dims = spritesheet.0.dimensions();

	let pixel_to_text_coord = |(x, y)| {
		let norm_x = x as f32 / image_dims.0 as f32;
		let norm_y = y as f32 / image_dims.1 as f32;
		(norm_x, norm_y)
	};

	for (text, &(ul, lr)) in sorted_iter
		.iter()
		.map(|(_index, text)| text)
		.zip(&spritesheet.1)
	{
		let (ulx, uly) = pixel_to_text_coord(ul);
		let (lrx, lry) = pixel_to_text_coord(lr);

		//small shift to avoid atlas bleeding
		const TINY_SHIFT: f32 = f32::EPSILON;
		let texture = GLvec4(
			ulx + TINY_SHIFT,
			uly + TINY_SHIFT,
			lrx - TINY_SHIFT,
			lry - TINY_SHIFT,
		);

		let width = (lr.0 - ul.0) as f32;
		let height = (lr.1 - ul.1) as f32;

		map.insert(
			text.name(),
			Instance {
				texture,
				scale: (width, height / text.frame_count() as f32).into(),
				rotation: 0f32.into(),
				..Default::default()
			},
		);
	}

	(spritesheet.0, map)
}
