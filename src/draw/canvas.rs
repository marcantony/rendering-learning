use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas::new_with_color(width, height, &Color::new(0.0, 0.0, 0.0))
    }

    pub fn new_with_color(width: usize, height: usize, color: &Color) -> Canvas {
        Canvas {
            width: width,
            height: height,
            data: vec![color.clone(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn write(&mut self, coords: (usize, usize), color: Color) -> Option<()> {
        let idx = self.to_idx(coords);
        idx.map(|i| {
            self.data[i] = color;
            ()
        })
    }

    pub fn at(&self, coords: (usize, usize)) -> Option<&Color> {
        let idx = self.to_idx(coords);
        idx.and_then(|i| self.data.get(i))
    }

    fn to_idx(&self, coords: (usize, usize)) -> Option<usize> {
        let (x, y) = coords;
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.width * coords.1 + coords.0)
        }
    }

    pub fn ppm(&self) -> String {
        const MAX_COLOR_VAL: i32 = 255;

        fn translate(color: f64) -> i32 {
            let val = (color * MAX_COLOR_VAL as f64).round() as i32;
            val.clamp(0, MAX_COLOR_VAL)
        }

        // header
        let mut s = String::from("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str(&MAX_COLOR_VAL.to_string());
        s.push('\n');

        // pixel data
        let pixel_data = self
            .data
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .flat_map(|color| {
                        let r = translate(color.r());
                        let g = translate(color.g());
                        let b = translate(color.b());

                        [r, g, b].into_iter().map(|v| v.to_string())
                    })
                    .reduce(|acc, i| {
                        let length_after_last_newline = acc
                            .rfind('\n')
                            .map(|i| acc.len() - i - 1)
                            .unwrap_or(acc.len());
                        if length_after_last_newline + i.len() + 1 > 70 {
                            // +1 for the space before the color data
                            acc + "\n" + &i
                        } else {
                            acc + " " + &i
                        }
                    })
                    .unwrap()
            })
            .collect::<Vec<String>>()
            .join("\n");
        s.push_str(&pixel_data);
        s.push('\n');

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::color::Color;

    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);

        for pixel in c.data {
            assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let r = Color::new(1.0, 0.0, 0.0);

        c.write((2, 3), r.clone());

        assert_eq!(c.at((2, 3)).expect("pixel is defined"), &r);
    }

    mod ppm {
        use super::*;

        #[test]
        fn constructing_ppm_header() {
            let c = Canvas::new(5, 3);
            let ppm = c.ppm();
            let lines = ppm.lines();

            let first3 = lines.take(3).collect::<Vec<_>>().join("\n");

            assert_eq!(
                first3,
                "P3
5 3
255"
            );
        }

        #[test]
        fn constructing_ppm_pixel_data() {
            let mut c = Canvas::new(5, 3);
            c.write((0, 0), Color::new(1.5, 0.0, 0.0));
            c.write((2, 1), Color::new(0.0, 0.5, 0.0));
            c.write((4, 2), Color::new(-0.5, 0.0, 1.0));

            let ppm = c.ppm();
            let lines = ppm.lines();

            let lines4to6 = lines.skip(3).take(3).collect::<Vec<_>>().join("\n");

            assert_eq!(
                lines4to6,
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            );
        }

        #[test]
        fn splitting_long_lines_in_ppm() {
            let c = Canvas::new_with_color(10, 2, &Color::new(1.0, 0.8, 0.6));

            let ppm = c.ppm();
            let lines = ppm.lines();

            let lines4to7 = lines.skip(3).take(4).collect::<Vec<_>>().join("\n");

            assert_eq!(
                lines4to7,
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"
            );
        }

        #[test]
        fn ppm_files_terminated_by_newline() {
            let c = Canvas::new(5, 3);

            let ppm = c.ppm();
            assert!(ppm.ends_with('\n'));
        }
    }
}
