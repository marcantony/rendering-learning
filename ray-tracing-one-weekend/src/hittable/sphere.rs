use crate::{
    aabb::AABB,
    hittable::{self, HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

#[derive(Clone)]
pub enum Center {
    Stationary(Point3),
    Moving(Point3, Point3),
}

#[derive(Clone)]
pub struct Sphere<M> {
    pub center: Center,
    pub radius: f64,
    pub material: M,
}

impl<M> Sphere<M> {
    fn center(&self, time: f64) -> Point3 {
        match &self.center {
            Center::Stationary(p) => p.clone(),
            Center::Moving(p1, p2) => p1 + time * (p2 - p1),
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    type Material = M;
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&M, HitRecord)> {
        let center = self.center(r.time);
        let oc = &r.origin - &center;
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
                let outward_normal = NormalizedVec3::from_normalized((&p - &center) / self.radius);
                let (normal, face) = hittable::calculate_face_normal(r, outward_normal.clone());
                (
                    &self.material,
                    HitRecord {
                        p,
                        normal,
                        t,
                        face,
                        uv: get_sphere_uv(&outward_normal),
                    },
                )
            })
        }
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        match &self.center {
            Center::Stationary(center) => {
                AABB::new_from_points(&(center - &rvec), &(center + &rvec))
            }
            Center::Moving(start, end) => {
                let start_box = AABB::new_from_points(&(start - &rvec), &(start + &rvec));
                let end_box = AABB::new_from_points(&(end - &rvec), &(end + &rvec));
                start_box.merge(&end_box)
            }
        }
    }
}

/// Given a point on the unit sphere, returns the UV coordinates
fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

    (
        phi / (2.0 * std::f64::consts::PI),
        theta / std::f64::consts::PI,
    )
}

#[cfg(test)]
mod tests {
    use float_cmp::{approx_eq, assert_approx_eq};

    use crate::{hittable::Face, material::Flat, vec3::Vec3};

    use super::*;

    fn test_sphere() -> Sphere<Flat> {
        Sphere {
            center: Center::Stationary(Point3::new(0.0, 0.0, 0.0)),
            radius: 1.0,
            material: Flat,
        }
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 2.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(sphere.hit(&ray, &Interval::nonnegative()).is_none());
    }

    #[test]
    fn a_ray_is_tangent_to_a_sphere() {
        let sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 1.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval::nonnegative()).unwrap().1;

        assert_eq!(hit.t, 5.0);
        assert_eq!(hit.face, Face::Front);
        assert_approx_eq!(&Vec3, &hit.normal, &Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn a_ray_goes_through_a_sphere() {
        let sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval::nonnegative()).unwrap().1;

        assert_eq!(hit.t, 4.0);
        assert_eq!(hit.face, Face::Front);
        assert_approx_eq!(&Vec3, &hit.normal, &Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn a_ray_starts_inside_a_sphere() {
        let sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval::nonnegative()).unwrap().1;

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.face, Face::Back);
        assert_approx_eq!(&Vec3, &hit.normal, &Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_outside_the_interval() {
        let sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval { min: 0.0, max: 1.0 });

        assert!(hit.is_none());
    }

    #[test]
    fn a_ray_intersects_a_sphere_bounding_the_interval() {
        let sphere = test_sphere();
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.hit(&ray, &Interval { min: 0.0, max: 4.0 });

        assert_eq!(hit.map(|h| h.1.t), Some(4.0));
    }

    mod uv {
        use super::*;

        macro_rules! uv_coord_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (point, (expected_u, expected_v)) = $value;
                        let (u, v) = get_sphere_uv(&point);
                        assert!(approx_eq!(f64, u, expected_u, epsilon = 0.01));
                        assert!(approx_eq!(f64, v, expected_v, epsilon = 0.01));
                    }
                )*
            };
        }

        uv_coord_tests! {
            pos_x: (Point3::new(1.0, 0.0, 0.0), (0.5, 0.5)),
            pos_y: (Point3::new(0.0, 1.0, 0.0), (0.5, 1.0)),
            pos_z: (Point3::new(0.0, 0.0, 1.0), (0.25, 0.5)),
            neg_x: (Point3::new(-1.0, 0.0, 0.0), (0.0, 0.5)),
            neg_y: (Point3::new(0.0, -1.0, 0.0), (0.5, 0.0)),
            neg_z: (Point3::new(0.0, 0.0, -1.0), (0.75, 0.5))
        }
    }
}
