use crate::math::util;

use super::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        self.object
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        util::are_equal(self.t, other.t) && self.object == other.object
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t(), 3.5);
        assert_eq!(i.object(), &s);
    }
}
