use crate::{draw::color::Color, math::tuple::Tuple3};

pub struct PointLight {
    pub position: Tuple3,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use crate::{draw::color::Color, math::tuple::Tuple3};

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple3::point(0.0, 0.0, 0.0);
        let light = PointLight {
            position: position.clone(),
            intensity: intensity.clone(),
        };

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
