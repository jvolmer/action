use std::fmt;

use itertools::Itertools;

pub struct Lattice<Type, const SIZE: usize> {
    pub sites: [[Type; SIZE]; SIZE],
}

impl<Type: Copy + Default, const SIZE: usize> Lattice<Type, SIZE> {
    pub fn new() -> Lattice<Type, SIZE> {
        Lattice {
            sites: [[Type::default(); SIZE]; SIZE],
        }
    }
}
impl<Type, const SIZE: usize> Lattice<Type, SIZE> {
    pub fn from(sites: [[Type; SIZE]; SIZE]) -> Lattice<Type, SIZE> {
        Lattice { sites }
    }
}
impl<Type: Clone, const SIZE: usize> Lattice<Type, SIZE> {
    pub fn site(&self, x: isize, y: isize) -> &Type {
        let length = self.sites.len() as isize;
        &self.sites[((x % length + length) % length) as usize]
            [((y % length + length) % length) as usize]
    }

    pub fn neighborhood(&self, x: isize, y: isize) -> Vec<&Type> {
        vec![
            self.site(x + 1, y),
            self.site(x - 1, y),
            self.site(x, y + 1),
            self.site(x, y - 1),
        ]
    }
    pub fn iter(&self) -> impl Iterator<Item = &Type> {
        self.sites.iter().flat_map(|x| x.iter())
    }
}
impl<Type: fmt::Display + fmt::Debug, const SIZE: usize> fmt::Display for Lattice<Type, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.sites.iter().map(|row| row.iter().join(" ")).join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_periodic_boundary_conditions() {
        let lattice = Lattice::from([['A', 'B'], ['C', 'D']]);

        assert_eq!(lattice.site(-3, 0), &'C');
        assert_eq!(lattice.site(-2, 0), &'A');
        assert_eq!(lattice.site(-1, 0), &'C');
        assert_eq!(lattice.site(1, 0), &'C');
        assert_eq!(lattice.site(2, 0), &'A');
        assert_eq!(lattice.site(3, 0), &'C');
        assert_eq!(lattice.site(4, 0), &'A');

        assert_eq!(lattice.site(0, -3), &'B');
        assert_eq!(lattice.site(0, -2), &'A');
        assert_eq!(lattice.site(0, -1), &'B');
        assert_eq!(lattice.site(0, 0), &'A');
        assert_eq!(lattice.site(0, 1), &'B');
        assert_eq!(lattice.site(0, 2), &'A');
        assert_eq!(lattice.site(0, 3), &'B');
        assert_eq!(lattice.site(0, 4), &'A');

        assert_eq!(lattice.site(1, 1), &'D');
    }

    #[test]
    fn gives_neighborhood() {
        assert_eq!(
            Lattice::from([['A', 'B'], ['C', 'D']]).neighborhood(0, 0),
            vec![&'C', &'C', &'B', &'B']
        );
    }

    #[test]
    fn iterates_over_sites() {
        assert_eq!(
            Lattice::from([['A', 'B'], ['C', 'D']])
                .iter()
                .collect::<Vec<&char>>(),
            vec![&'A', &'B', &'C', &'D']
        );
    }

    #[test]
    fn displays_lattice() {
        assert_eq!(
            format!("{}", Lattice::from([['A', 'B'], ['C', 'D']])),
            "A B\nC D"
        );
    }
}
