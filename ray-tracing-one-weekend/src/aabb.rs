use crate::{interval::Interval, ray::Ray, vec3::Point3};

/// An axis-aligned bounding box
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

    /// Tests a ray against an AABB. Returns an intersection interval if the ray hit.
    pub fn hit(&self, ray: &Ray) -> Option<Interval> {
        let (xtmin, xtmax) = intersect_axis(&self.x, ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = intersect_axis(&self.y, ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = intersect_axis(&self.z, ray.origin.z(), ray.direction.z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin <= tmax {
            Some(Interval {
                min: tmin,
                max: tmax,
            })
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::*;

    macro_rules! aabb_intersect_tests {
        ($($name:ident: $value:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let (ray, expected) = $value;

                    let aabb = AABB::new_from_points(&Point3::new(2.0, 2.0, 2.0), &Point3::new(4.0, 4.0, 4.0));

                    assert_eq!(aabb.hit(&ray), expected);
                }
            )*
        };
    }

    aabb_intersect_tests! {
        intersect_x: (Ray::new(Point3::new(0.0, 3.0, 3.0), Vec3::new(1.0, 0.0, 0.0)), Some(Interval {min: 2.0, max: 4.0})),
        intersect_y: (Ray::new(Point3::new(3.0, 5.0, 3.0), Vec3::new(0.0, -1.0, 0.0)), Some(Interval {min: 1.0, max: 3.0})),
        intersect_z: (Ray::new(Point3::new(3.0, 3.0, 1.5), Vec3::new(0.0, 0.0, 1.0)), Some(Interval {min: 0.5, max: 2.5})),
        miss: (Ray::new(Point3::new(5.0, 5.0, 5.0), Vec3::new(0.0, 0.0, 1.0)), None)
    }
}
