use crate::lattice::lattice::Lattice;

use super::spin::{Spin, SpinValue};

#[derive(Debug, PartialEq)]
pub struct Correlation(pub i64);

#[derive(Debug, PartialEq)]
pub struct Magnetization(pub i64);

pub type SpinLattice<const SIZE: usize> = Lattice<Spin, SIZE>;
impl<const SIZE: usize> SpinLattice<SIZE> {
    pub fn magnetization(&self) -> Magnetization {
        Magnetization(self.iter().map(|s| s.value()).sum::<SpinValue>().0)
    }
}
