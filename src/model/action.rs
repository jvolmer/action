use crate::{
    model::spin::{Spin, SpinValue},
    monte_carlo::boltzmann_weight::BoltzmannWeight,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Action(i64);

impl Action {
    pub fn new(value: i64) -> Self {
        Action(value)
    }
    pub fn local(spin: &Spin, neighborhood: Vec<&Spin>) -> Self {
        let spin_sum: SpinValue = neighborhood.into_iter().map(|s| s.value()).sum();
        Action(4 * spin_sum.correlate_with(spin.value()).0)
    }
    pub fn boltzmann_weight(self) -> BoltzmannWeight {
        BoltzmannWeight::new((-self.0 as f64).exp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_local_action_from_spin_and_neighborhood() {
        assert_eq!(Action::local(&Spin::Up, vec![&Spin::Up]), Action(4));
        assert_eq!(Action::local(&Spin::Down, vec![&Spin::Up]), Action(-4));
        assert_eq!(
            Action::local(&Spin::Up, vec![&Spin::Up, &Spin::Up]),
            Action(8)
        );
        assert_eq!(
            Action::local(&Spin::Down, vec![&Spin::Up, &Spin::Up]),
            Action(-8)
        );
    }
}
