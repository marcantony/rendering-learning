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
            .map(|object| {
                object
                    .intersect(ray)
                    .into_iter()
                    .map(|t| Intersection::new(t, object.as_ref()))
            })
            .flatten()
            .collect::<Vec<_>>();
        intersections.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        intersections
    }

    fn shade_hit(&self, comps: &Precomputation<dyn Object>, remaining: usize) -> Option<Color> {
        self.lights
            .iter()
            .map(|light| {
                let shadowed = self.is_shadowed(&comps.over_point, light);

                let surface_color = lighting(
                    &comps.object.material(),
                    comps.object,
                    &comps.point,
                    light,
                    &comps.eye_v,
                    &comps.normal_v,
                    shadowed,
                );

                let reflected_color = self.reflected_color(comps, remaining);

                &surface_color + &reflected_color
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

    fn is_shadowed(&self, point: &Point3d, light: &PointLight) -> bool {
        let v = &light.position - point;
        let distance = v.mag();
        let direction = v.norm();

        direction
            .and_then(|d| {
                let r = Ray {
                    origin: point.clone(),
                    direction: d,
                };
                let intersections = self.intersect(&r);
                intersect::hit(&intersections).map(|hit| hit.t() < distance)
            })
            .unwrap_or(false)
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
    use crate::{draw::color, math::vector::Vec3d};

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
            assert!(!w.is_shadowed(&p, &w.lights[0]));
        }

        #[test]
        fn shadow_when_an_object_is_between_point_and_light() {
            let w = World::basic();
            let p = Point3d::new(10.0, -10.0, 10.0);
            assert!(w.is_shadowed(&p, &w.lights[0]));
        }

        #[test]
        fn no_shadow_when_an_object_is_behind_the_light() {
            let w = World::basic();
            let p = Point3d::new(-20.0, 20.0, -20.0);
            assert!(!w.is_shadowed(&p, &w.lights[0]));
        }

        #[test]
        fn shadow_when_an_object_is_behind_the_point() {
            let w = World::basic();
            let p = Point3d::new(-2.0, 2.0, -2.0);
            assert!(!w.is_shadowed(&p, &w.lights[0]));
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
}
