use crate::lattice::lattice::Lattice;

use super::spin::{Spin, SpinValue};

#[derive(Debug, PartialEq)]
pub struct Correlation(i64);
impl Correlation {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    pub fn get(&self) -> i64 {
        self.0
    }
}

#[derive(Debug)]
pub struct Magnetization(SpinValue);
impl Magnetization {
    pub fn from(value: SpinValue) -> Self {
        Self(value)
    }
    pub fn get(&self) -> i64 {
        self.0.get()
    }
}

pub type SpinLattice<const SIZE: usize> = Lattice<Spin, SIZE>;
impl<const SIZE: usize> SpinLattice<SIZE> {
    pub fn magnetization(&self) -> Magnetization {
        Magnetization::from(self.iter().map(|s| s.value()).sum::<SpinValue>())
    }
}
