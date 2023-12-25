use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            data: vec![Color::new(0.0, 0.0, 0.0); width * height]
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
}
