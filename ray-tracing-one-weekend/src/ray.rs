use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_at_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::{assert_approx_eq, ApproxEq, F64Margin};

    use super::*;

    impl ApproxEq for &Ray {
        type Margin = F64Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let m = margin.into();
            self.origin.approx_eq(&other.origin, m) && self.direction.approx_eq(&other.direction, m)
        }
    }

    #[test]
    fn compute_a_point_from_a_distance() {
        let r = Ray::new(Point3::new(2.0, 3.0, 4.0), Vec3::new(1.0, 0.0, 0.0));

        assert_approx_eq!(&Point3, &r.at(0.0), &Point3::new(2.0, 3.0, 4.0));
        assert_approx_eq!(&Point3, &r.at(1.0), &Point3::new(3.0, 3.0, 4.0));
        assert_approx_eq!(&Point3, &r.at(-1.0), &Point3::new(1.0, 3.0, 4.0));
        assert_approx_eq!(&Point3, &r.at(2.5), &Point3::new(4.5, 3.0, 4.0));
    }
}
