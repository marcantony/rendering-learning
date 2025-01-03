use crate::{interval::Interval, ray::Ray, vec3::Point3};

/// An axis-aligned bounding box
#[derive(Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    /// Generates a bounding box from the extrema points
    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval {
                min: a.x(),
                max: b.x(),
            }
        } else {
            Interval {
                min: b.x(),
                max: a.x(),
            }
        };
        let y = if a.y() <= b.y() {
            Interval {
                min: a.y(),
                max: b.y(),
            }
        } else {
            Interval {
                min: b.y(),
                max: a.y(),
            }
        };
        let z = if a.z() <= b.z() {
            Interval {
                min: a.z(),
                max: b.z(),
            }
        } else {
            Interval {
                min: b.z(),
                max: a.z(),
            }
        };

        AABB { x, y, z }
    }

    pub fn empty() -> Self {
        AABB {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    /// Tests a ray against an AABB. Returns an intersection interval if the ray hit.
    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let (xtmin, xtmax) = intersect_axis(&self.x, ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = intersect_axis(&self.y, ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = intersect_axis(&self.z, ray.origin.z(), ray.direction.z());

        let tmin = xtmin.max(ytmin).max(ztmin).max(ray_t.min);
        let tmax = xtmax.min(ytmax).min(ztmax).min(ray_t.max);

        tmin < tmax
    }

    /// Creates a new AABB which contains self and the input AABB
    pub fn merge(&self, other: &AABB) -> Self {
        let x = self.x.merge(&other.x);
        let y = self.y.merge(&other.y);
        let z = self.z.merge(&other.z);
        AABB { x, y, z }
    }
}

fn intersect_axis(i: &Interval, origin: f64, speed: f64) -> (f64, f64) {
    let t0 = (i.min - origin) / speed;
    let t1 = (i.max - origin) / speed;

    if t0 < t1 {
        (t0, t1)
    } else {
        (t1, t0)
    }
}

impl Default for AABB {
    fn default() -> Self {
        AABB::empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::*;

    macro_rules! aabb_intersect_tests {
        ($($name:ident: $value:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let (ray, ray_t, expected) = $value;

                    let aabb = AABB::new_from_points(&Point3::new(2.0, 2.0, 2.0), &Point3::new(4.0, 4.0, 4.0));

                    assert_eq!(aabb.hit(&ray, &ray_t), expected);
                }
            )*
        };
    }

    aabb_intersect_tests! {
        intersect_x: (Ray::new(Point3::new(0.0, 3.0, 3.0), Vec3::new(1.0, 0.0, 0.0)), Interval::universe(), true),
        intersect_y: (Ray::new(Point3::new(3.0, 5.0, 3.0), Vec3::new(0.0, -1.0, 0.0)), Interval::universe(), true),
        intersect_z: (Ray::new(Point3::new(3.0, 3.0, 1.5), Vec3::new(0.0, 0.0, 1.0)), Interval::universe(), true),
        miss: (Ray::new(Point3::new(5.0, 5.0, 5.0), Vec3::new(0.0, 0.0, 1.0)), Interval::universe(), false),
        intersect_in_constrained_interval: (Ray::new(Point3::new(0.0, 3.0, 3.0), Vec3::new(1.0, 0.0, 0.0)), Interval {min: 2.0, max: 3.0}, true),
        intersect_outside_interval: (Ray::new(Point3::new(0.0, 3.0, 3.0), Vec3::new(1.0, 0.0, 0.0)), Interval {min: 1.0, max: 1.9}, false),
        miss_outside_interval: (Ray::new(Point3::new(5.0, 5.0, 5.0), Vec3::new(0.0, 0.0, 1.0)), Interval::universe(), false)
    }
}
