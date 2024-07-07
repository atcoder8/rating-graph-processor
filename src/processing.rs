use image::{imageops, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};

/// Size of X(formerly Twitter) header image.
const HEADER_SIZE: (u32, u32) = (1500, 500);

/// Opaque white color.
const WHITE: Rgba<u8> = Rgba([255; 4]);

/// Creates an image with an opaque white background added.
pub(crate) fn create_opaque_image(img: &RgbaImage) -> RgbaImage {
    let mut composited_image = RgbaImage::from_pixel(img.width(), img.height(), WHITE);
    imageops::overlay(&mut composited_image, img, 0, 0);

    composited_image
}

/// Creates a resized image that fits into a box while keeping the aspect ratio.
fn create_boxed_image<I>(
    image: &I,
    box_shape: (u32, u32),
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
    I: GenericImageView,
    I::Pixel: 'static,
    <I::Pixel as Pixel>::Subpixel: 'static,
{
    let scaling_ratio = (HEADER_SIZE.0 as f64 / image.width() as f64)
        .min(HEADER_SIZE.1 as f64 / image.height() as f64);
    let scaled_w = ((image.width() as f64 * scaling_ratio) as u32).min(box_shape.0);
    let scaled_h = ((image.height() as f64 * scaling_ratio) as u32).min(box_shape.1);

    imageops::resize(image, scaled_w, scaled_h, imageops::FilterType::CatmullRom)
}

/// Adds a white background and resizes the image that fits into the header size.
pub(crate) fn create_header_image(img: &RgbaImage) -> RgbaImage {
    let scaled_image = create_boxed_image(img, HEADER_SIZE);

    let top_margin = (HEADER_SIZE.0 - scaled_image.width()) / 2;
    let left_margin = (HEADER_SIZE.1 - scaled_image.height()) / 2;

    let mut composited_image = RgbaImage::from_pixel(HEADER_SIZE.0, HEADER_SIZE.1, WHITE);
    imageops::overlay(
        &mut composited_image,
        &scaled_image,
        top_margin as i64,
        left_margin as i64,
    );

    composited_image
}
