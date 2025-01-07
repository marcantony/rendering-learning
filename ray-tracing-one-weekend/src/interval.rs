use std::ops::Add;

#[derive(PartialEq, Debug, Clone)]
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

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// Creates a new interval tightly enclosing self and the input interval
    pub fn merge(&self, other: &Interval) -> Self {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Interval { min, max }
    }
}

impl Add<f64> for &Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Add<&Interval> for &f64 {
    type Output = Interval;

    fn add(self, rhs: &Interval) -> Self::Output {
        rhs + *self
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

    #[test]
    fn expanding_an_interval() {
        let i = Interval { min: 1.0, max: 2.0 };
        let expanded = i.expand(2.0);

        assert_eq!(expanded, Interval { min: 0.0, max: 3.0 })
    }
}
