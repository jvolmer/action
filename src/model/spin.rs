use std::{fmt, iter::Sum, ops::Add};

use super::observables::Correlation;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Spin {
    Up,
    Down,
}
impl Spin {
    pub fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    pub fn value(&self) -> SpinValue {
        match self {
            Self::Up => SpinValue(1),
            Self::Down => SpinValue(-1),
        }
    }
}
impl fmt::Display for Spin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "+"),
            Self::Down => write!(f, "-"),
        }
    }
}
impl Default for Spin {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Debug, PartialEq)]
pub struct SpinValue(pub i64);
impl SpinValue {
    pub fn correlate_with(&self, value: SpinValue) -> Correlation {
        Correlation(self.0 * value.0)
    }
}
impl Add<SpinValue> for SpinValue {
    type Output = SpinValue;
    fn add(self, other: SpinValue) -> SpinValue {
        SpinValue(self.0 + other.0)
    }
}
impl Sum<SpinValue> for SpinValue {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = SpinValue>,
    {
        iter.fold(SpinValue(0), |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_spins() {
        assert_eq!(Spin::Up.value() + Spin::Down.value(), SpinValue(0));
        assert_eq!(Spin::Down.value() + Spin::Up.value(), SpinValue(0));
        assert_eq!(Spin::Up.value() + Spin::Up.value(), SpinValue(2));
        assert_eq!(Spin::Down.value() + Spin::Down.value(), SpinValue(-2));
    }

    #[test]
    fn flips_spins() {
        assert_eq!(Spin::Up.flip(), Spin::Down);
        assert_eq!(Spin::Down.flip(), Spin::Up);
    }

    #[test]
    fn up_has_value_one() {
        assert_eq!(Spin::Up.value(), SpinValue(1));
    }

    #[test]
    fn down_has_value_minus_one() {
        assert_eq!(Spin::Down.value(), SpinValue(-1));
    }

    #[test]
    fn spin_values_correlate_with_each_other() {
        assert_eq!(
            Spin::Up.value().correlate_with(Spin::Up.value()),
            Correlation(1)
        );
        assert_eq!(SpinValue(2).correlate_with(SpinValue(4)), Correlation(8));
    }

    #[test]
    fn spin_values_can_be_summed() {
        assert_eq!(
            [Spin::Up.value(), Spin::Up.value(), Spin::Up.value()]
                .into_iter()
                .sum::<SpinValue>(),
            SpinValue(3)
        );
    }
}
