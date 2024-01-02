use crate::correlation::Correlation;
use crate::model::spin::{Spin, SpinSum};

#[derive(PartialEq, Debug, Clone)]
pub struct Action {
    value: i64,
}

impl Action {
    pub fn new(value: i64) -> Self {
        Action { value }
    }
    pub fn local(spin: &Spin, neighborhood: &Vec<Spin>) -> Self {
        let spin_sum: SpinSum = neighborhood.into_iter().sum();
        Action {
            value: 4 * Correlation::of(spin, &spin_sum).value(),
        }
    }
    pub fn boltzmann_weight(self) -> f64 {
        (-self.value as f64).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_local_action_from_spin_and_neighborhood() {
        assert_eq!(
            Action::local(&Spin::Up, &vec![Spin::Up]),
            Action { value: 4 }
        );
        assert_eq!(
            Action::local(&Spin::Down, &vec![Spin::Up]),
            Action { value: -4 }
        );
        assert_eq!(
            Action::local(&Spin::Up, &vec![Spin::Up, Spin::Up]),
            Action { value: 8 }
        );
        assert_eq!(
            Action::local(&Spin::Down, &vec![Spin::Up, Spin::Up]),
            Action { value: -8 }
        );
    }
}
