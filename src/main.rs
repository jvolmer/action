pub mod correlation;
pub mod lattice;
pub mod model;
pub mod monte_carlo;

use model::action::Action;
use model::spin::Spin;
use monte_carlo::metropolis_step::MonteCarloSimulation;

fn main() {
    let old_spin = Spin::Up;
    let action = Action::local(&old_spin, &vec![Spin::Up]);
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
