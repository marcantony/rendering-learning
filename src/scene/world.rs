use std::collections::HashSet;

use by_address::ByAddress;

use crate::{
    draw::color::{self, Color},
    math::{
        matrix::{InvertibleMatrix, SquareMatrix},
        point::Point3d,
    },
    scene::{
        material::{Material, Surface},
        transformation,
    },
};

use super::{
    intersect::{self, Intersection, Precomputation},
    light::PointLight,
    material::lighting,
    object::{sphere::Sphere, Object},
    ray::Ray,
};

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<PointLight>,
    pub max_reflection_depth: usize,
}

impl World {
    pub fn basic() -> Self {
        World {
            objects: basic_spheres()
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn Object>)
                .collect(),
            lights: vec![basic_light()],
            max_reflection_depth: 5,
        }
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<dyn Object>> {
        let mut intersections = self
            .objects
            .iter()
            .map(|object| object.intersect(ray))
            .flatten()
            .collect::<Vec<_>>();
        intersect::sort(&mut intersections);

        intersections
    }

    fn shade_hit(&self, comps: &Precomputation<dyn Object>, remaining: usize) -> Option<Color> {
        self.lights
            .iter()
            .map(|light| {
                let shadow_attenuation = self.shadow_attenuation(&comps.over_point, light);

                let surface_color = lighting(
                    comps.object,
                    &comps.point,
                    light,
                    &comps.eye_v,
                    &comps.normal_v,
                    shadow_attenuation,
                );

                let reflected_color = self.reflected_color(comps, remaining);
                let refracted_color = self.refracted_color(comps, remaining);

                let m = comps.object.material();
                if m.reflectivity > 0.0 && m.transparency > 0.0 {
                    let reflectance = comps.schlick();
                    &surface_color
                        + &(&(&reflected_color * reflectance)
                            + &(&refracted_color * (1.0 - reflectance)))
                } else {
                    &surface_color + &(&reflected_color + &refracted_color)
                }
            })
            .reduce(|acc, c| &acc + &c)
    }

    fn color_at_internal(&self, ray: &Ray, remaining: usize) -> Color {
        let xs = self.intersect(ray);

        intersect::hit(&xs)
            .and_then(|h| {
                let comps = h.prepare_computations(ray, &xs);
                self.shade_hit(&comps, remaining)
            })
            .unwrap_or(color::black())
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_internal(ray, self.max_reflection_depth)
    }

    fn shadow_attenuation(&self, point: &Point3d, light: &PointLight) -> f64 {
        let v = &light.position - point;
        let distance = v.mag();
        let direction = v.norm();

        direction
            .map(|d| {
                let r = Ray {
                    origin: point.clone(),
                    direction: d,
                };
                let intersections = self.intersect(&r);
                let mut seen = HashSet::<ByAddress<&dyn Object>>::new();
                intersections
                    .iter()
                    .filter(|i| i.t() > 0.0 && i.t() < distance)
                    .take_while(|i| seen.insert(ByAddress(i.object())))
                    .map(|i| i.object().material().transparency)
                    .product()
            })
            .unwrap_or(1.0)
    }

    fn reflected_color(&self, comps: &Precomputation<dyn Object>, remaining: usize) -> Color {
        if remaining == 0 || comps.object.material().reflectivity == 0.0 {
            color::black()
        } else {
            let reflect_ray = Ray::new(comps.over_point.clone(), (*comps.reflect_v).clone());
            let color = self.color_at_internal(&reflect_ray, remaining - 1);
            &color * comps.object.material().reflectivity
        }
    }

    fn refracted_color(&self, comps: &Precomputation<dyn Object>, remaining: usize) -> Color {
        if remaining == 0 || comps.object.material().transparency == 0.0 {
            color::black()
        } else {
            // Snell's law: n_ratio = n1/n2
            let n_ratio = comps.refraction_exiting / comps.refraction_entering;
            let cos_i = comps.eye_v.dot(&comps.normal_v);
            let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

            if sin2_t > 1.0 {
                // Total internal reflection
                color::black()
            } else {
                let cos_t = f64::sqrt(1.0 - sin2_t);
                let direction =
                    &(&*comps.normal_v * (n_ratio * cos_i - cos_t)) - &(&*comps.eye_v * n_ratio);
                let refract_ray = Ray::new(comps.under_point.clone(), direction);
                &self.color_at_internal(&refract_ray, remaining - 1)
                    * comps.object.material().transparency
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            lights: Default::default(),
            max_reflection_depth: 5,
        }
    }
}

fn basic_light() -> PointLight {
    PointLight {
        position: Point3d::new(-10.0, 10.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    }
}

fn basic_spheres() -> Vec<Sphere> {
    let s1 = Sphere::new(
        InvertibleMatrix::try_from(SquareMatrix::<4>::identity()).unwrap(),
        Material {
            surface: Surface::Color(Color::new(0.8, 1.0, 0.6)),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        },
    );
    let s2 = Sphere::new(
        InvertibleMatrix::try_from(transformation::scaling(0.5, 0.5, 0.5)).unwrap(),
        Default::default(),
    );

    vec![s1, s2]
}

#[cfg(test)]
mod tests {
    use crate::{draw::color, math::vector::Vec3d, scene::object::plane::Plane};

    use super::*;

    const TEST_DEPTH: usize = 5;

    #[test]
    fn create_a_world() {
        let w: World = Default::default();

        assert!(w.objects.is_empty());
        assert_eq!(w.lights, vec![]);
    }

    // Commenting out this test... too hard to figure out comparing equality of dyn Object...
    // #[test]
    // fn the_default_world() {
    //     let w = World::basic();

    //     assert_eq!(w.lights, vec![basic_light()]);
    //     assert_eq!(w.objects, basic_spheres());
    // }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::basic();
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::basic();
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap().as_ref();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&comps, TEST_DEPTH);

        color::test_utils::assert_colors_approx_equal(
            &c.unwrap(),
            &Color::new(0.38066, 0.47583, 0.2855),
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::basic();
        w.lights = vec![PointLight {
            position: Point3d::new(0.0, 0.25, 0.0),
            intensity: color::white(),
        }];
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
        let shape = w.objects[1].as_ref();
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&comps, TEST_DEPTH);

        color::test_utils::assert_colors_approx_equal(
            &c.unwrap(),
            &Color::new(0.90498, 0.90498, 0.90498),
        );
    }

    #[test]
    fn shading_when_there_are_no_lights() {
        let mut w = World::basic();
        w.lights = vec![];
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap().as_ref();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&comps, TEST_DEPTH);

        assert_eq!(c, None);
    }

    #[test]
    fn shading_with_two_lights() {
        let mut w = World::basic();
        w.lights = vec![w.lights[0].clone(), w.lights[0].clone()];
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap().as_ref();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&comps, TEST_DEPTH);

        color::test_utils::assert_colors_approx_equal(
            &c.unwrap(),
            &Color::new(0.38066 * 2.0, 0.47583 * 2.0, 0.2855 * 2.0),
        );
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let shape = Sphere {
            transform: InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 10.0))
                .unwrap(),
            ..Default::default()
        };
        let w = World {
            lights: vec![PointLight {
                position: Point3d::new(0.0, 0.0, -10.0),
                intensity: color::white(),
            }],
            objects: vec![Box::<Sphere>::new(Default::default()), Box::new(shape)],
            ..Default::default()
        };
        let r = Ray {
            origin: Point3d::new(0.0, 0.0, 5.0),
            direction: Vec3d::new(0.0, 0.0, 1.0),
        };
        let i = Intersection::new(4.0, w.objects[1].as_ref() as &dyn Object);

        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&comps, TEST_DEPTH);

        assert_eq!(c, Some(Color::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::basic();
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 1.0, 0.0));
        let c = w.color_at_internal(&r, TEST_DEPTH);

        assert_eq!(c, color::black());
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::basic();
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
        let c = w.color_at_internal(&r, TEST_DEPTH);

        color::test_utils::assert_colors_approx_equal(&c, &Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut spheres = basic_spheres();
        let outer = &mut spheres[0];
        outer.material.ambient = 1.0;
        let inner = &mut spheres[1];
        inner.material.ambient = 1.0;

        let w = World {
            objects: spheres
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn Object>)
                .collect(),
            lights: vec![basic_light()],
            ..Default::default()
        };
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.75), Vec3d::new(0.0, 0.0, -1.0));

        let c = w.color_at_internal(&r, TEST_DEPTH);
        let inner_surface = &w.objects[1].material().surface;

        assert!(matches!(inner_surface, Surface::Color(col) if col == &c));
    }

    mod shadow {
        use super::*;

        #[test]
        fn no_shadow_when_nothing_collinear_with_point_and_light() {
            let w = World::basic();
            let p = Point3d::new(0.0, 10.0, 0.0);
            assert_eq!(w.shadow_attenuation(&p, &w.lights[0]), 1.0);
        }

        #[test]
        fn shadow_when_an_object_is_between_point_and_light() {
            let w = World::basic();
            let p = Point3d::new(10.0, -10.0, 10.0);
            assert_eq!(w.shadow_attenuation(&p, &w.lights[0]), 0.0);
        }

        #[test]
        fn no_shadow_when_an_object_is_behind_the_light() {
            let w = World::basic();
            let p = Point3d::new(-20.0, 20.0, -20.0);
            assert_eq!(w.shadow_attenuation(&p, &w.lights[0]), 1.0);
        }

        #[test]
        fn no_shadow_when_an_object_is_behind_the_point() {
            let w = World::basic();
            let p = Point3d::new(-2.0, 2.0, -2.0);
            assert_eq!(w.shadow_attenuation(&p, &w.lights[0]), 1.0);
        }

        #[test]
        fn partial_shadow_when_somewhat_transparent_objects_are_between_point_and_light() {
            let mut spheres = basic_spheres();
            spheres[0].material.transparency = 0.5;
            spheres[1].material.transparency = 1.0;
            let w = World {
                objects: spheres
                    .into_iter()
                    .map(|s| Box::new(s) as Box<dyn Object>)
                    .collect(),
                lights: vec![basic_light()],
                ..Default::default()
            };
            let p = Point3d::new(10.0, -10.0, 10.0);
            assert_eq!(w.shadow_attenuation(&p, &w.lights[0]), 0.5);
        }
    }

    mod reflect {
        use crate::scene::object::plane::Plane;

        use super::*;

        #[test]
        fn reflected_color_for_a_nonreflective_material() {
            let mut spheres = basic_spheres();
            let inner = &mut spheres[1];
            inner.material.ambient = 1.0;

            let w = World {
                objects: spheres
                    .into_iter()
                    .map(|s| Box::new(s) as Box<dyn Object>)
                    .collect(),
                lights: vec![basic_light()],
                ..Default::default()
            };
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let i = Intersection::new(1.0, w.objects[1].as_ref());

            let comps = i.prepare_computations(&r, &vec![]);
            let color = w.reflected_color(&comps, TEST_DEPTH);

            assert_eq!(color, color::black());
        }

        #[test]
        fn reflected_color_for_reflective_material() {
            let shape = Plane {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -1.0, 0.0))
                    .unwrap(),
                material: Material {
                    reflectivity: 0.5,
                    ..Default::default()
                },
            };
            let mut w = World::basic();
            w.objects.push(Box::new(shape));

            let sqrt2 = std::f64::consts::SQRT_2;
            let r = Ray::new(
                Point3d::new(0.0, 0.0, -3.0),
                Vec3d::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
            );
            let i = Intersection::new(sqrt2, w.objects[2].as_ref());

            let comps = i.prepare_computations(&r, &vec![]);
            let color = w.reflected_color(&comps, TEST_DEPTH);

            color::test_utils::assert_colors_approx_equal(
                &color,
                &Color::new(0.19033, 0.23791, 0.14274),
            );
        }

        #[test]
        fn shade_hit_with_a_reflective_material() {
            let shape = Plane {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -1.0, 0.0))
                    .unwrap(),
                material: Material {
                    reflectivity: 0.5,
                    ..Default::default()
                },
            };
            let mut w = World::basic();
            w.objects.push(Box::new(shape));

            let sqrt2 = std::f64::consts::SQRT_2;
            let r = Ray::new(
                Point3d::new(0.0, 0.0, -3.0),
                Vec3d::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
            );
            let i = Intersection::new(sqrt2, w.objects[2].as_ref());

            let comps = i.prepare_computations(&r, &vec![]);
            let color = w.shade_hit(&comps, TEST_DEPTH).unwrap();

            color::test_utils::assert_colors_approx_equal(
                &color,
                &Color::new(0.87675, 0.92434, 0.82917),
            );
        }

        #[test]
        fn color_at_with_mutually_reflective_surfaces() {
            let light = PointLight {
                position: Point3d::new(0.0, 0.0, 0.0),
                intensity: color::white(),
            };
            let lower = Plane {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -1.0, 0.0))
                    .unwrap(),
                material: Material {
                    reflectivity: 1.0,
                    ..Default::default()
                },
            };
            let upper = Plane {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, 1.0, 0.0))
                    .unwrap(),
                material: Material {
                    reflectivity: 1.0,
                    ..Default::default()
                },
            };
            let w = World {
                lights: vec![light],
                objects: vec![Box::new(lower), Box::new(upper)],
                ..Default::default()
            };
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 1.0, 0.0));

            w.color_at_internal(&r, TEST_DEPTH);
        }

        #[test]
        fn reflected_color_at_maximum_recursive_depth() {
            let shape = Plane {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -1.0, 0.0))
                    .unwrap(),
                material: Material {
                    reflectivity: 0.5,
                    ..Default::default()
                },
            };
            let mut w = World::basic();
            w.objects.push(Box::new(shape));

            let sqrt2 = std::f64::consts::SQRT_2;
            let r = Ray::new(
                Point3d::new(0.0, 0.0, -3.0),
                Vec3d::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
            );
            let i = Intersection::new(sqrt2, w.objects[2].as_ref());

            let comps = i.prepare_computations(&r, &vec![]);
            let color = w.reflected_color(&comps, 0);

            color::test_utils::assert_colors_approx_equal(&color, &color::black());
        }
    }

    mod refract {
        use crate::scene::{
            object::{plane::Plane, sphere},
            pattern::test_utils::MockPattern,
        };

        use super::*;

        #[test]
        fn the_refracted_color_with_an_opaque_surface() {
            let w = World::basic();
            let shape = w.objects[0].as_ref();
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = vec![Intersection::new(4.0, shape), Intersection::new(6.0, shape)];

            let comps = xs[0].prepare_computations(&r, &xs);
            let c = w.refracted_color(&comps, 5);

            assert_eq!(c, color::black());
        }

        #[test]
        fn the_refracted_color_at_the_maximum_recursive_depth() {
            let shape = sphere::glass_sphere();
            let mut w = World::basic();
            w.objects[0] = Box::new(shape);
            let shape_ref = w.objects[0].as_ref();
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = vec![
                Intersection::new(4.0, shape_ref),
                Intersection::new(6.0, shape_ref),
            ];

            let comps = xs[0].prepare_computations(&r, &xs);
            let c = w.refracted_color(&comps, 0);

            assert_eq!(c, color::black());
        }

        #[test]
        fn the_refracted_color_under_total_internal_reflection() {
            let mut spheres = basic_spheres();
            let shape = &mut spheres[0];
            shape.material.transparency = 1.0;
            shape.material.refractive_index = 1.5;
            let mut w = World::basic();
            w.objects = spheres
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn Object>)
                .collect();
            let shape_ref = w.objects[0].as_ref();

            let t = std::f64::consts::SQRT_2 / 2.0;
            let r = Ray::new(Point3d::new(0.0, 0.0, t), Vec3d::new(0.0, 1.0, 0.0));
            let xs = vec![
                Intersection::new(-t, shape_ref),
                Intersection::new(t, shape_ref),
            ];

            let comps = xs[1].prepare_computations(&r, &xs);
            let c = w.refracted_color(&comps, 5);

            assert_eq!(c, color::black());
        }

        #[test]
        fn the_refracted_color_with_a_refracted_ray() {
            let mut spheres = basic_spheres();
            let a = &mut spheres[0];
            a.material.ambient = 1.0;
            a.material.surface = Surface::Pattern(Box::new(MockPattern {
                transform: InvertibleMatrix::identity(),
            }));
            let b = &mut spheres[1];
            b.material.transparency = 1.0;
            b.material.refractive_index = 1.5;
            let mut w = World::basic();
            w.objects = spheres
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn Object>)
                .collect();
            let a_ref = w.objects[0].as_ref();
            let b_ref = w.objects[1].as_ref();

            let r = Ray::new(Point3d::new(0.0, 0.0, 0.1), Vec3d::new(0.0, 1.0, 0.0));
            let xs: Vec<_> = vec![
                (-0.9899, a_ref),
                (-0.4899, b_ref),
                (0.4899, b_ref),
                (0.9899, a_ref),
            ]
            .into_iter()
            .map(|(t, o)| Intersection::new(t, o))
            .collect();

            let comps = xs[2].prepare_computations(&r, &xs);
            let c = w.refracted_color(&comps, 5);

            color::test_utils::assert_colors_approx_equal(&c, &Color::new(0.0, 0.99888, 0.04721));
        }

        #[test]
        fn shade_hit_with_a_transparent_material() {
            let mut w = World::basic();
            let floor = Plane {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -1.0, 0.0))
                    .unwrap(),
                material: Material {
                    transparency: 0.5,
                    refractive_index: 1.5,
                    ..Default::default()
                },
            };
            let ball = Sphere {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -3.5, -0.5))
                    .unwrap(),
                material: Material {
                    surface: Surface::Color(color::red()),
                    ambient: 0.5,
                    ..Default::default()
                },
            };
            w.objects.push(Box::new(floor));
            w.objects.push(Box::new(ball));

            let sqrt2 = std::f64::consts::SQRT_2;
            let r = Ray::new(
                Point3d::new(0.0, 0.0, -3.0),
                Vec3d::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
            );
            let xs = vec![Intersection::new(sqrt2, w.objects[2].as_ref())];

            let comps = xs[0].prepare_computations(&r, &xs);
            let color = w.shade_hit(&comps, 5).unwrap();

            color::test_utils::assert_colors_approx_equal(
                &color,
                &Color::new(1.12546, 0.68642, 0.68642),
            );
        }
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::basic();
        let floor = Plane {
            transform: InvertibleMatrix::try_from(transformation::translation(0.0, -1.0, 0.0))
                .unwrap(),
            material: Material {
                transparency: 0.5,
                refractive_index: 1.5,
                reflectivity: 0.5,
                ..Default::default()
            },
        };
        let ball = Sphere {
            transform: InvertibleMatrix::try_from(transformation::translation(0.0, -3.5, -0.5))
                .unwrap(),
            material: Material {
                surface: Surface::Color(color::red()),
                ambient: 0.5,
                ..Default::default()
            },
        };
        w.objects.push(Box::new(floor));
        w.objects.push(Box::new(ball));

        let sqrt2 = std::f64::consts::SQRT_2;
        let r = Ray::new(
            Point3d::new(0.0, 0.0, -3.0),
            Vec3d::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
        );
        let xs = vec![Intersection::new(sqrt2, w.objects[2].as_ref())];

        let comps = xs[0].prepare_computations(&r, &xs);
        let color = w.shade_hit(&comps, 5).unwrap();

        color::test_utils::assert_colors_approx_equal(
            &color,
            &Color::new(1.11500, 0.69643, 0.69243),
        );
    }
}
