pub mod correlation;
pub mod model;
pub mod monte_carlo;

use model::action::Action;
use model::spin::{Spin, SpinSum};
use monte_carlo::metropolis_step::MonteCarloSimulation;

fn main() {
    let old_spin = Spin::Up;
    let neighborhood = SpinSum::new(1);
    let action = Action::local(&old_spin, &neighborhood);
    println!("Old Spin: {:?}, Action: {:?}", old_spin, action);

    let mut simulation = MonteCarloSimulation::new(569);
    let (up_count, down_count) = (0..1000)
        .map(|_| {
            let step = simulation.step(action.clone());
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
