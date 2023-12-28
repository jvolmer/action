use crate::correlation::Observable;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub enum Spin {
    Up,
    Down,
}
impl Add for Spin {
    type Output = SpinSum;
    fn add(self, other: Self) -> SpinSum {
        SpinSum {
            value: self.value() + other.value(),
        }
    }
}
impl Spin {
    pub fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
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

#[derive(PartialEq, Debug)]
pub struct SpinSum {
    value: i64,
}
impl SpinSum {
    pub fn new(value: i64) -> Self {
        SpinSum { value }
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
