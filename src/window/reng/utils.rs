#[allow(dead_code)]

pub fn to_char_slice_mut<T>(array: &mut [T]) -> &mut [u8] {
	let data_ptr = array.as_ptr() as *mut u8;

	unsafe { std::slice::from_raw_parts_mut(data_ptr, std::mem::size_of_val(array)) }
}

pub fn to_char_slice<T>(array: &[T]) -> &[u8] {
	let data_ptr = array.as_ptr() as *const u8;

	unsafe { std::slice::from_raw_parts(data_ptr, std::mem::size_of_val(array)) }
}

type Coords = (u32, u32);
type Dimens = (u32, u32);

pub fn create_spritesheet(
	mut images: Vec<image::RgbaImage>,
) -> (image::RgbaImage, Vec<(Coords, Dimens)>) {
	use image::GenericImage;
	use image::GenericImageView;
	let mut dyn_image = image::DynamicImage::new_rgba8(1, 1);

	let mut corners = vec![(0u32, 0u32)];
	let mut placed_images: Vec<((u32, u32), (u32, u32))> = vec![];

	images.sort_by_key(|e| -((e.height() * e.width()) as i32));

	for image in &images {
		let mut best_extension: Option<(usize, (u32, u32))> = None;
		corners.sort_by_key(|c| c.1);
		'corner_loop: for corner in corners.iter().enumerate() {
			let x_coord = corner.1 .0 + image.width();
			let y_coord = corner.1 .1 + image.height();

			let size_dims = dyn_image.dimensions();
			let x_ext = x_coord as i32 - size_dims.0 as i32;
			let y_ext = y_coord as i32 - size_dims.1 as i32;

			for placed_image in &placed_images {
				let placed_ul = placed_image.0;
				let placed_lr = placed_image.1;

				let corner_ul = corner.1;
				let corner_lr = (x_coord, y_coord);

				if corner_ul.0 < placed_lr.0
					&& corner_lr.0 > placed_ul.0
					&& corner_ul.1 < placed_lr.1
					&& corner_lr.1 > placed_ul.1
				{
					continue 'corner_loop;
				}
			}

			let extension = (
				if x_ext > 0 { x_ext as u32 } else { 0 },
				if y_ext > 0 { y_ext as u32 } else { 0 },
			);

			if let Some(best_ext) = best_extension {
				let dims = dyn_image.dimensions();
				let compute_ext_size =
					|ext: (u32, u32)| ext.0 * dims.1 + ext.1 * dims.0 + ext.0 * ext.1;
				if compute_ext_size(best_ext.1) > compute_ext_size(extension) {
					best_extension = Some((corner.0, extension));
				}
			} else {
				best_extension = Some((corner.0, extension));
			}
		}

		let position = best_extension.unwrap();
		let corner = corners.remove(position.0);

		if position.1 != (0u32, 0u32) {
			//Red background makes debugging easier
			let new = image::ImageBuffer::from_pixel(
				dyn_image.dimensions().0 + position.1 .0,
				dyn_image.dimensions().1 + position.1 .1,
				image::Rgba([1., 0., 0., 1.]),
			);

			let mut grown = image::DynamicImage::from(new);

			grown.copy_from(&dyn_image, 0, 0).unwrap();
			dyn_image = grown;
		}

		dyn_image.copy_from(image, corner.0, corner.1).unwrap();
		let dims = (
			corner.0 + image.dimensions().0,
			corner.1 + image.dimensions().1,
		);

		placed_images.push((corner, dims));

		//1px padding to avoid bleeding
		corners.push((dims.0, corner.1));
		corners.push((corner.0, dims.1));
	}

	(dyn_image.into_rgba8(), placed_images)
}
