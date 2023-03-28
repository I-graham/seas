use super::{glsl::*, reng::*, types::*};
use strum::IntoEnumIterator;

pub fn load_textures() -> (image::RgbaImage, TextureMap) {
	let mut map = TextureMap::default();

	let mut rgba_images = Texture::iter()
		.map(|text_name| {
			let file_name = format!("assets/{}.png", <&'static str>::from(text_name));
			image::open(file_name)
				.expect("Unable to open image texture asset.")
				.into_rgba8()
		})
		.collect::<Vec<_>>();

	let img_size = |img: &image::RgbaImage| (img.height() * img.width()) as i32;

	let mut sorted_iter = Texture::iter().enumerate().collect::<Vec<_>>();
	sorted_iter.sort_by_key(|(index, _text)| -img_size(&rgba_images[*index]));

	rgba_images.sort_by_key(|e| -img_size(e));

	let spritesheet = utils::create_spritesheet(rgba_images);

	let image_dims = spritesheet.0.dimensions();

	let pixel_to_text_coord = |(x, y)| {
		let norm_x = x as f32 / image_dims.0 as f32;
		let norm_y = y as f32 / image_dims.1 as f32;
		(norm_x, norm_y)
	};

	for (text, &(rotated, ul, lr)) in sorted_iter
		.iter()
		.map(|(_index, text)| text)
		.zip(&spritesheet.1)
	{
		let (ulx, uly) = pixel_to_text_coord(ul);
		let (lrx, lry) = pixel_to_text_coord(lr);

		let texture = GLvec4(ulx, uly, lrx, lry);

		let raw_ratio = (lr.0 as f32 - ul.0 as f32) / (lr.1 as f32 - ul.1 as f32);
		let ratio = if rotated {
			raw_ratio.recip()
		} else {
			raw_ratio
		};

		map.insert(
			*text,
			Instance {
				texture,
				scale: (text.frame_count() as f32 * ratio, 1.0).into(),
				rotation: if rotated { -90. } else { 0. }.into(),
				..Default::default()
			},
		);
	}

	(spritesheet.0, map)
}
