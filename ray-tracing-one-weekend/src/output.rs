use std::io::{Result, Seek, Write};

use image::{ExtendedColorType, ImageFormat, ImageResult};

use crate::camera::Canvas;

pub fn output_ppm(canvas: &Canvas, out: &mut impl Write) -> Result<()> {
    writeln!(out, "P3")?;
    writeln!(out, "{} {}", canvas.width, canvas.height)?;
    writeln!(out, "255")?;

    canvas
        .pixel_data()
        .map(|c| c.write_ppm(out))
        .collect::<Result<()>>()
}

pub fn output_image<W: Write + Seek>(
    canvas: &Canvas,
    image_format: ImageFormat,
    out: &mut W,
) -> ImageResult<()> {
    let pixels = canvas
        .pixel_data()
        .map(|c| c.linear_to_srgb().to_u8())
        .flat_map(|(r, g, b)| [r, g, b].into_iter())
        .collect::<Vec<_>>();

    image::write_buffer_with_format(
        out,
        &pixels,
        canvas.width as u32,
        canvas.height as u32,
        ExtendedColorType::Rgb8,
        image_format
    )
}
