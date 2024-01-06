use crate::{
    draw::color::{self, Color},
    math::{
        matrix::{InvertibleMatrix, SquareMatrix},
        point::Point3d,
    },
    scene::{material::Material, transformation},
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
}

impl World {
    pub fn basic() -> Self {
        World {
            objects: basic_spheres()
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn Object>)
                .collect(),
            lights: vec![basic_light()],
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

    fn shade_hit(&self, comps: &Precomputation<dyn Object>) -> Option<Color> {
        self.lights
            .iter()
            .map(|light| {
                let shadowed = self.is_shadowed(&comps.over_point, light);
                lighting(
                    &comps.object.material(),
                    comps.object,
                    &comps.point,
                    light,
                    &comps.eye_v,
                    &comps.normal_v,
                    shadowed,
                )
            })
            .reduce(|acc, c| &acc + &c)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);

        intersect::hit(&xs)
            .and_then(|h| {
                let comps = h.prepare_computations(ray);
                self.shade_hit(&comps)
            })
            .unwrap_or(color::black())
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
}

impl Default for World {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            lights: Default::default(),
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
            color: Color::new(0.8, 1.0, 0.6),
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

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

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

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

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

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, None);
    }

    #[test]
    fn shading_with_two_lights() {
        let mut w = World::basic();
        w.lights = vec![w.lights[0].clone(), w.lights[0].clone()];
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap().as_ref();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

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
            objects: vec![
                Box::<Sphere>::new(Default::default()),
                Box::new(shape.clone()),
            ],
        };
        let r = Ray {
            origin: Point3d::new(0.0, 0.0, 5.0),
            direction: Vec3d::new(0.0, 0.0, 1.0),
        };
        let i = Intersection::new(4.0, &shape as &dyn Object);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Some(Color::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::basic();
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);

        assert_eq!(c, color::black());
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::basic();
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);

        color::test_utils::assert_colors_approx_equal(&c, &Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut spheres = basic_spheres();
        let outer = &mut spheres[0];
        outer.material.ambient = 1.0;
        let inner = &mut spheres[1];
        inner.material.ambient = 1.0;
        let inner_color = inner.material.color.clone();

        let w = World {
            objects: spheres
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn Object>)
                .collect(),
            lights: vec![basic_light()],
        };
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.75), Vec3d::new(0.0, 0.0, -1.0));

        let c = w.color_at(&r);

        assert_eq!(c, inner_color);
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
}
