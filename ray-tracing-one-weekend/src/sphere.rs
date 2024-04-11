use crate::{
    hittable::{self, HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{NormalizedVec3, Point3},
};

pub struct Sphere<M> {
    pub center: Point3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = &r.origin - &self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let discriminant_sqrt = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range
            let r_l = (-half_b - discriminant_sqrt) / a;
            let r_u = (-half_b + discriminant_sqrt) / a;
            let root = if ray_t.contains(r_l) {
                Some(r_l)
            } else if ray_t.contains(r_u) {
                Some(r_u)
            } else {
                None
            };

            root.map(|t| {
                let p = r.at(t);
                let outward_normal = NormalizedVec3::from((&p - &self.center) / self.radius);
                let (normal, face) = hittable::calculate_face_normal(r, outward_normal);
                HitRecord {
                    p,
                    normal,
                    t,
                    face,
                    material: &mut self.material,
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::{hittable::Face, material::Flat, vec3::Vec3};

    use super::*;

    fn test_sphere() -> Sphere<Flat> {
        Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Flat,
        }
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let mut sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 2.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(sphere.hit(&ray, &Interval::nonnegative()).is_none());
    }

    #[test]
    fn a_ray_is_tangent_to_a_sphere() {
        let mut sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 1.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval::nonnegative()).unwrap();

        assert_eq!(hit.t, 5.0);
        assert_eq!(hit.face, Face::Front);
        assert_approx_eq!(&Vec3, &hit.normal, &Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn a_ray_goes_through_a_sphere() {
        let mut sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval::nonnegative()).unwrap();

        assert_eq!(hit.t, 4.0);
        assert_eq!(hit.face, Face::Front);
        assert_approx_eq!(&Vec3, &hit.normal, &Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn a_ray_starts_inside_a_sphere() {
        let mut sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval::nonnegative()).unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.face, Face::Back);
        assert_approx_eq!(&Vec3, &hit.normal, &Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_outside_the_interval() {
        let mut sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval { min: 0.0, max: 1.0 });

        assert!(hit.is_none());
    }

    #[test]
    fn a_ray_intersects_a_sphere_bounding_the_interval() {
        let mut sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval { min: 0.0, max: 4.0 });

        assert_eq!(hit.map(|h| h.t), Some(4.0));
    }
}
