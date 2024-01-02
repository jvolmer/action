use crate::correlation::Observable;
use std::{fmt, iter::Sum, ops::Add};

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
}
impl Add for Spin {
    type Output = SpinSum;
    fn add(self, other: Self) -> SpinSum {
        SpinSum {
            value: self.value() + other.value(),
        }
    }
}
impl Observable for Spin {
    fn value(&self) -> i64 {
        match self {
            Self::Up => 1,
            Self::Down => -1,
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

#[derive(PartialEq, Debug)]
pub struct SpinSum {
    value: i64,
}
impl SpinSum {
    pub fn new(value: i64) -> Self {
        SpinSum { value }
    }
}
impl Add<Spin> for SpinSum {
    type Output = SpinSum;
    fn add(self, other: Spin) -> SpinSum {
        SpinSum {
            value: self.value() + other.value(),
        }
    }
}
impl<'a> Sum<&'a Spin> for SpinSum {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Spin>,
    {
        iter.fold(SpinSum { value: 0 }, |acc, &x| acc + x)
    }
}
impl Observable for SpinSum {
    fn value(&self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_spins() {
        assert_eq!(Spin::Up + Spin::Down, SpinSum { value: 0 });
        assert_eq!(Spin::Down + Spin::Up, SpinSum { value: 0 });
        assert_eq!(Spin::Up + Spin::Up, SpinSum { value: 2 });
        assert_eq!(Spin::Down + Spin::Down, SpinSum { value: -2 });
    }

    #[test]
    fn flips_spins() {
        assert_eq!(Spin::Up.flip(), Spin::Down);
        assert_eq!(Spin::Down.flip(), Spin::Up);
    }

    #[test]
    fn up_has_value_one() {
        assert_eq!(Spin::Up.value(), 1);
    }

    #[test]
    fn down_has_value_minus_one() {
        assert_eq!(Spin::Down.value(), -1);
    }
}
