pub mod lattice;
pub mod model;
pub mod monte_carlo;

use crate::model::action::Action;
use crate::model::observables::SpinLattice;
use crate::model::spin::Spin;
use crate::monte_carlo::metropolis_step::MonteCarloSimulation;

fn main() {
    let mut simulation = MonteCarloSimulation::new(569);

    const SIZE: usize = 5;
    let mut lattice = SpinLattice::<SIZE>::new();

    println!(
        "Old lattice:\n{}, m = {:?}",
        lattice,
        lattice.magnetization()
    );

    for i in 1..SIZE {
        for j in 1..SIZE {
            let action = Action::local(
                lattice.site(i as isize, j as isize),
                lattice.neighborhood(i as isize, j as isize),
            );
            lattice.sites[i][j] = simulation.step(action).update(lattice.sites[i][j]);
        }
    }

    println!(
        "New lattice:\n{}, m = {:?}",
        lattice,
        lattice.magnetization()
    );

    let old_spin = Spin::Up;
    let action = Action::local(&old_spin, vec![&Spin::Up]);
    println!("Old Spin: {:?}, Action: {:?}", old_spin, action);

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
