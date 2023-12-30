use crate::math::util;

use super::sphere::Sphere;

#[derive(Debug, Clone)]
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

pub fn hit<'a, 'b>(intersections: &'a[Intersection<'b>]) -> Option<&'a Intersection<'b>> {
    intersections.iter().fold(None, |acc, i| {
        if i.t() >= 0.0 {
            acc
                .map(|lowest| if lowest.t() < i.t() { lowest } else { i })
                .or(Some(i))
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::unit();

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t(), 3.5);
        assert_eq!(i.object(), &s);
    }

    mod hit {
        use super::*;

        #[test]
        fn hit_when_all_intersections_have_positive_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(1.0, &s);
            let i2 = Intersection::new(2.0, &s);
            let xs = vec![i2.clone(), i1.clone()];

            let i = hit(&xs);

            assert_eq!(i, Some(&i1));
        }

        #[test]
        fn hit_when_some_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-1.0, &s);
            let i2 = Intersection::new(1.0, &s);
            let xs = vec![i2.clone(), i1.clone()];

            let i = hit(&xs);

            assert_eq!(i, Some(&i2));
        }

        #[test]
        fn hit_when_all_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-2.0, &s);
            let i2 = Intersection::new(-1.0, &s);
            let xs = vec![i2.clone(), i1.clone()];

            let i = hit(&xs);

            assert_eq!(i, None);
        }

        #[test]
        fn hit_is_always_lowest_nonnegative_intersection() {
            let s = Sphere::unit();
            let i1 = Intersection::new(5.0, &s);
            let i2 = Intersection::new(7.0, &s);
            let i3 = Intersection::new(-3.0, &s);
            let i4 = Intersection::new(2.0, &s);
            let xs = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];

            let i = hit(&xs);

            assert_eq!(i, Some(&i4));
        }
    }
}
