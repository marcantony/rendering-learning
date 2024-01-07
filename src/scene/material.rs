use crate::{
    draw::color::{self, Color},
    math::{point::Point3d, vector::NormalizedVec3d},
};

use super::{light::PointLight, object::Object, pattern::Pattern};

pub enum Surface {
    Color(Color),
    Pattern(Box<dyn Pattern>),
}

pub struct Material {
    pub surface: Surface,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectivity: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            surface: Surface::Color(Color::new(1.0, 1.0, 1.0)),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectivity: 0.0,
        }
    }
}

pub fn lighting(
    material: &Material,
    object: &dyn Object,
    point: &Point3d,
    light: &PointLight,
    eyev: &NormalizedVec3d,
    normalv: &NormalizedVec3d,
    in_shadow: bool,
) -> Color {
    let effective_color = match &material.surface {
        Surface::Color(c) => c * &light.intensity,
        Surface::Pattern(p) => &p.at_object(object, point) * &light.intensity,
    };
    let lightv = (&light.position - point).norm().unwrap();

    let ambient = &effective_color * material.ambient;

    let light_dot_normal = lightv.dot(normalv);

    let (diffuse, specular) = if in_shadow || light_dot_normal < 0.0 {
        (color::black(), color::black())
    } else {
        let diff = &(&effective_color * material.diffuse) * light_dot_normal;

        let reflectv = -&lightv.reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        (
            diff,
            if reflect_dot_eye <= 0.0 {
                color::black()
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                &(&light.intensity * material.specular) * factor
            },
        )
    };

    return &(&ambient + &diffuse) + &specular;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_material() {
        let m: Material = Default::default();

        assert!(matches!(m.surface, Surface::Color(c) if c == Color::new(1.0, 1.0, 1.0)));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
        assert_eq!(m.reflectivity, 0.0);
    }

    mod lighting {
        use crate::{
            math::{matrix::InvertibleMatrix, vector::Vec3d},
            scene::{light::PointLight, object::sphere::Sphere, pattern::stripe::Stripe},
        };

        use super::*;

        fn setup() -> (Material, Point3d) {
            (Default::default(), Point3d::new(0.0, 0.0, 0.0))
        }

        #[test]
        fn lighting_with_eye_between_light_and_surface() {
            let (m, position) = setup();
            let eyev = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let normalv = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 0.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &position,
                &light,
                &eyev,
                &normalv,
                false,
            );
            assert_eq!(result, Color::new(1.9, 1.9, 1.9));
        }

        #[test]
        fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
            let (m, position) = setup();
            let t = std::f64::consts::SQRT_2 / 2.0;
            let eyev = NormalizedVec3d::try_from(Vec3d::new(0.0, t, -t)).unwrap();
            let normalv = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 0.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &position,
                &light,
                &eyev,
                &normalv,
                false,
            );
            assert_eq!(result, Color::new(1.0, 1.0, 1.0));
        }

        #[test]
        fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
            let (m, position) = setup();
            let eyev = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let normalv = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 10.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &position,
                &light,
                &eyev,
                &normalv,
                false,
            );
            color::test_utils::assert_colors_approx_equal(
                &result,
                &Color::new(0.7364, 0.7364, 0.7364),
            );
        }

        #[test]
        fn lighting_with_eye_in_path_of_reflection_vector() {
            let (m, position) = setup();
            let t = std::f64::consts::SQRT_2 / 2.0;
            let eyev = NormalizedVec3d::try_from(Vec3d::new(0.0, -t, -t)).unwrap();
            let normalv = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 10.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &position,
                &light,
                &eyev,
                &normalv,
                false,
            );
            color::test_utils::assert_colors_approx_equal(
                &result,
                &Color::new(1.6364, 1.6364, 1.6364),
            );
        }

        #[test]
        fn lighting_with_light_behind_surface() {
            let (m, position) = setup();
            let eyev = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let normalv = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 0.0, 10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &position,
                &light,
                &eyev,
                &normalv,
                false,
            );
            assert_eq!(result, Color::new(0.1, 0.1, 0.1));
        }

        #[test]
        fn lighting_with_surface_in_shadow() {
            let (m, position) = setup();
            let eyev = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let normalv = NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 0.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };
            let in_shadow = true;

            let result = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &position,
                &light,
                &eyev,
                &normalv,
                in_shadow,
            );
            assert_eq!(result, Color::new(0.1, 0.1, 0.1));
        }

        #[test]
        fn lighting_with_a_pattern_applied() {
            let m = Material {
                surface: Surface::Pattern(Box::new(Stripe {
                    a: color::white(),
                    b: color::black(),
                    transform: InvertibleMatrix::identity(),
                })),
                ambient: 1.0,
                diffuse: 0.0,
                specular: 0.0,
                ..Default::default()
            };
            let eyev = NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap();
            let normalv = NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap();
            let light = PointLight {
                position: Point3d::new(0.0, 0.0, -10.0),
                intensity: color::white(),
            };

            let c1 = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &Point3d::new(0.9, 0.0, 0.0),
                &light,
                &eyev,
                &normalv,
                false,
            );
            let c2 = lighting(
                &m,
                &Sphere::unit() as &dyn Object,
                &Point3d::new(1.1, 0.0, 0.0),
                &light,
                &eyev,
                &normalv,
                false,
            );

            assert_eq!(c1, color::white());
            assert_eq!(c2, color::black());
        }
    }
}
