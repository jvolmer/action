use rand::{Rng, SeedableRng};

use crate::model::action::Action;
use crate::model::spin::Spin;

pub struct MonteCarloSimulation {
    // TODO this only has to a uniform distribution [0,1)
    rng: rand_pcg::Pcg64Mcg,
}
impl MonteCarloSimulation {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: rand_pcg::Pcg64Mcg::seed_from_u64(seed),
        }
    }
    pub fn step(&mut self, action: Action) -> MetropolisStep {
        action.boltzmann_weight().step(self.rng.gen())
    }
}

#[derive(Debug, PartialEq)]
pub struct MetropolisStep {
    is_accepted: bool,
}
impl MetropolisStep {
    pub fn new(is_accepted: bool) -> Self {
        Self { is_accepted }
    }
    pub fn accept() -> Self {
        Self { is_accepted: true }
    }
    pub fn reject() -> Self {
        Self { is_accepted: false }
    }
    pub fn update(self, spin: Spin) -> Spin {
        if self.is_accepted {
            spin.flip()
        } else {
            spin
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_action_is_always_accepted() {
        assert_eq!(
            Action::new(-8).boltzmann_weight().step(1.),
            MetropolisStep::accept()
        );
        assert_eq!(
            Action::new(-8).boltzmann_weight().step(0.),
            MetropolisStep::accept()
        );
        assert_eq!(
            Action::new(-8).boltzmann_weight().step(0.5),
            MetropolisStep::accept()
        );
    }

    #[test]
    fn positive_action_is_accepted_for_small_random_numbers() {
        assert_eq!(
            Action::new(2).boltzmann_weight().step(1.),
            MetropolisStep::reject()
        );
        assert_eq!(
            Action::new(2).boltzmann_weight().step(0.5),
            MetropolisStep::reject()
        );
        assert_eq!(
            Action::new(2).boltzmann_weight().step(0.),
            MetropolisStep::accept()
        );
    }

    #[test]
    fn flips_spin_in_accepted_step() {
        assert_eq!(MetropolisStep::accept().update(Spin::Up), Spin::Down);
    }

    #[test]
    fn leaves_spin_unchanged_in_rejected_step() {
        assert_eq!(MetropolisStep::reject().update(Spin::Up), Spin::Up);
    }
}
