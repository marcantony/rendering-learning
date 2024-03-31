use crate::{draw::color::Color, math::point::Point3d};

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    pub position: Point3d,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use crate::draw::color::Color;

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point3d::new(0.0, 0.0, 0.0);
        let light = PointLight {
            position: position.clone(),
            intensity: intensity.clone(),
        };

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
