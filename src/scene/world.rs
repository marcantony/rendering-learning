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
    ray::Ray,
    sphere::Sphere,
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

impl World {
    pub fn basic() -> Self {
        World {
            objects: basic_spheres(),
            light: Some(basic_light()),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = self
            .objects
            .iter()
            .map(|object| object.intersect(ray))
            .flatten()
            .flatten()
            .collect::<Vec<_>>();
        intersections.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        intersections
    }

    pub fn shade_hit(&self, comps: &Precomputation) -> Color {
        lighting(
            comps.object.material(),
            &comps.point,
            self.light.as_ref().unwrap(),
            &comps.eye_v,
            &comps.normal_v,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(h) = intersect::hit(&xs) {
            let comps = h.prepare_computations(ray);
            self.shade_hit(&comps)
        } else {
            color::black()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            light: Default::default(),
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
    use crate::{
        draw::color,
        math::{util, vector::Vec3d},
    };

    use super::*;

    #[test]
    fn create_a_world() {
        let w: World = Default::default();

        assert!(w.objects.is_empty());
        assert_eq!(w.light, None);
    }

    #[test]
    fn the_default_world() {
        let w = World::basic();

        assert_eq!(w.light, Some(basic_light()));
        assert_eq!(w.objects, basic_spheres());
    }

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
        let shape = w.objects.first().unwrap();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_colors_approx_equal(&c, &Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::basic();
        w.light = Some(PointLight {
            position: Point3d::new(0.0, 0.25, 0.0),
            intensity: color::white(),
        });
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_colors_approx_equal(&c, &Color::new(0.90498, 0.90498, 0.90498));
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

        assert_colors_approx_equal(&c, &Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = World::basic();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let inner_color = inner.material.color.clone();
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.75), Vec3d::new(0.0, 0.0, -1.0));

        let c = w.color_at(&r);

        assert_eq!(c, inner_color);
    }

    fn assert_colors_approx_equal(a: &Color, b: &Color) {
        assert!(util::test::are_within_tolerance(a.r(), b.r(), 1e-5));
        assert!(util::test::are_within_tolerance(a.g(), b.g(), 1e-5));
        assert!(util::test::are_within_tolerance(a.b(), b.b(), 1e-5));
    }
}
