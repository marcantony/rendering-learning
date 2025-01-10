use std::io::{Result, Write};

use crate::camera::Canvas;

pub fn output_ppm(canvas: &Canvas, out: &mut impl Write) -> Result<()> {
    writeln!(out, "P3")?;
    writeln!(out, "{} {}", canvas.width, canvas.height)?;
    writeln!(out, "255")?;

    canvas
        .data
        .iter()
        .map(|c| c.write_ppm(out))
        .collect::<Result<()>>()
}
