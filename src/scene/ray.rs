use crate::math::{matrix::SquareMatrix, point::Point3d, vector::Vec3d};

pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vec3d) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3d {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3d {
        &self.direction
    }

    pub fn position(&self, t: f64) -> Point3d {
        &self.origin + &(&self.direction * t)
    }

    pub fn transform(&self, transform: &SquareMatrix<4>) -> Self {
        Ray::new(transform * self.origin(), transform * self.direction())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_and_query_a_ray() {
        let origin = Point3d::new(1.0, 2.0, 3.0);
        let direction = Vec3d::new(4.0, 5.0, 6.0);

        let r = Ray::new(origin.clone(), direction.clone());

        assert_eq!(r.origin(), &origin);
        assert_eq!(r.direction(), &direction);
    }

    #[test]
    fn compute_a_point_from_a_distance() {
        let r = Ray::new(Point3d::new(2.0, 3.0, 4.0), Vec3d::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point3d::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point3d::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point3d::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point3d::new(4.5, 3.0, 4.0));
    }

    mod transform {

        use crate::scene::transformation;

        use super::*;

        #[test]
        fn translating_a_ray() {
            let r = Ray::new(Point3d::new(1.0, 2.0, 3.0), Vec3d::new(0.0, 1.0, 0.0));
            let m = transformation::translation(3.0, 4.0, 5.0);

            let r2 = r.transform(&m);

            assert_eq!(r2.origin(), &Point3d::new(4.0, 6.0, 8.0));
            assert_eq!(r2.direction(), &Vec3d::new(0.0, 1.0, 0.0));
        }

        #[test]
        fn scaling_a_ray() {
            let r = Ray::new(Point3d::new(1.0, 2.0, 3.0), Vec3d::new(0.0, 1.0, 0.0));
            let m = transformation::scaling(2.0, 3.0, 4.0);

            let r2 = r.transform(&m);

            assert_eq!(r2.origin(), &Point3d::new(2.0, 6.0, 12.0));
            assert_eq!(r2.direction(), &Vec3d::new(0.0, 3.0, 0.0));
        }
    }
}
