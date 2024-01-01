use crate::math::{matrix::SquareMatrix, tuple::Tuple3};

pub struct Ray {
    origin: Tuple3,
    direction: Tuple3,
}

impl Ray {
    pub fn new(origin: Tuple3, direction: Tuple3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Tuple3 {
        &self.origin
    }

    pub fn direction(&self) -> &Tuple3 {
        &self.direction
    }

    pub fn position(&self, t: f64) -> Tuple3 {
        &self.origin + &(&self.direction * t)
    }

    pub fn transform(&self, transform: &SquareMatrix<4>) -> Self {
        Ray::new(transform * self.origin(), transform * self.direction())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::tuple::Tuple3;

    use super::*;

    #[test]
    fn create_and_query_a_ray() {
        let origin = Tuple3::point(1.0, 2.0, 3.0);
        let direction = Tuple3::vec(4.0, 5.0, 6.0);

        let r = Ray::new(origin.clone(), direction.clone());

        assert_eq!(r.origin(), &origin);
        assert_eq!(r.direction(), &direction);
    }

    #[test]
    fn compute_a_point_from_a_distance() {
        let r = Ray::new(Tuple3::point(2.0, 3.0, 4.0), Tuple3::vec(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple3::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple3::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple3::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple3::point(4.5, 3.0, 4.0));
    }

    mod transform {

        use crate::scene::transformation;

        use super::*;

        #[test]
        fn translating_a_ray() {
            let r = Ray::new(Tuple3::point(1.0, 2.0, 3.0), Tuple3::vec(0.0, 1.0, 0.0));
            let m = transformation::translation(3.0, 4.0, 5.0);

            let r2 = r.transform(&m);

            assert_eq!(r2.origin(), &Tuple3::point(4.0, 6.0, 8.0));
            assert_eq!(r2.direction(), &Tuple3::vec(0.0, 1.0, 0.0));
        }

        #[test]
        fn scaling_a_ray() {
            let r = Ray::new(Tuple3::point(1.0, 2.0, 3.0), Tuple3::vec(0.0, 1.0, 0.0));
            let m = transformation::scaling(2.0, 3.0, 4.0);

            let r2 = r.transform(&m);

            assert_eq!(r2.origin(), &Tuple3::point(2.0, 6.0, 12.0));
            assert_eq!(r2.direction(), &Tuple3::vec(0.0, 3.0, 0.0));
        }
    }
}
