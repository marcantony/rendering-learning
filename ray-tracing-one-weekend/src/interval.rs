pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Self {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn nonnegative() -> Self {
        Interval {
            min: 0.0,
            max: f64::INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn universe_interval_contains_everything() {
        let i = Interval::universe();

        assert!(i.contains(f64::MAX));
        assert!(i.contains(f64::MIN));
        assert!(i.surrounds(f64::MAX));
        assert!(i.surrounds(f64::MIN));
    }

    #[test]
    fn interval_contains_and_surrounds_value() {
        let i = Interval {
            min: -10.0,
            max: 10.0,
        };

        assert!(i.contains(0.0));
        assert!(i.surrounds(0.0));
    }

    #[test]
    fn interval_surrounds_but_does_not_contain_boundary_value() {
        let i = Interval {
            min: -10.0,
            max: 10.0,
        };

        assert!(i.contains(10.0));
        assert!(!i.surrounds(10.0));
    }

    #[test]
    fn intervals_are_directional() {
        let i = Interval {
            min: 10.0,
            max: 0.0,
        };

        assert!(!i.contains(5.0));
    }
}
