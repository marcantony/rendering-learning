use std::io::{self, BufWriter, Result, Write};

fn main() -> Result<()> {
    let image_width: usize = 256;
    let image_height: usize = 256;

    let mut writer = BufWriter::new(io::stdout().lock());

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0 as f64;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            writeln!(writer, "{} {} {}", ir, ig, ib)?
        }
    }

    Ok(())
}
