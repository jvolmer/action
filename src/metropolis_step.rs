use crate::action::Action;
use crate::spin::Spin;

#[derive(Debug, PartialEq)]
pub struct MetropolisStep {
    is_accepted: bool,
}
impl MetropolisStep {
    // TODO rand is uniformly distributed in [0,1)
    pub fn from(action: Action, rand: f64) -> Self {
        // TODO add temperature
        MetropolisStep {
            is_accepted: action.boltzmann_weight() >= rand,
        }
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
            MetropolisStep::from(Action::new(-8), 1.),
            MetropolisStep { is_accepted: true }
        );
        assert_eq!(
            MetropolisStep::from(Action::new(-8), 0.),
            MetropolisStep { is_accepted: true }
        );
        assert_eq!(
            MetropolisStep::from(Action::new(-8), 0.5),
            MetropolisStep { is_accepted: true }
        );
    }

    #[test]
    fn positive_action_is_accepted_for_small_random_numbers() {
        assert_eq!(
            MetropolisStep::from(Action::new(2), 1.0),
            MetropolisStep { is_accepted: false }
        );
        assert_eq!(
            MetropolisStep::from(Action::new(2), 0.5),
            MetropolisStep { is_accepted: false }
        );
        assert_eq!(
            MetropolisStep::from(Action::new(2), 0.0),
            MetropolisStep { is_accepted: true }
        );
    }

    #[test]
    fn flips_spin_in_accepted_step() {
        assert_eq!(
            MetropolisStep { is_accepted: true }.update(Spin::Up),
            Spin::Down
        );
    }

    #[test]
    fn leaves_spin_unchanged_in_rejected_step() {
        assert_eq!(
            MetropolisStep { is_accepted: false }.update(Spin::Up),
            Spin::Up
        );
    }
}
