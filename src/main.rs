use action::Action;
use metropolis_step::MetropolisStep;
pub mod action;
pub mod correlation;
pub mod metropolis_step;
pub mod spin;

use rand::{Rng, SeedableRng};
use spin::{Spin, SpinSum};

fn main() {
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(569);
    let old_spin = Spin::Up;
    let neighborhood = SpinSum::new(1);
    let action = Action::local(&old_spin, &neighborhood);
    println!("Old Spin: {:?}, Action: {:?}", old_spin, action);

    let (up_count, down_count) = (0..1000)
        .map(|_| {
            let step = MetropolisStep::from(action.clone(), rng.gen());
            step.update(old_spin.clone())
        })
        .fold((0, 0), |(up_count, down_count), spin| {
            if spin == Spin::Up {
                (up_count + 1, down_count)
            } else {
                (up_count, down_count + 1)
            }
        });
    println!("New Spin: Up {}, Down {}", up_count, down_count);
}
