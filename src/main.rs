use action::model::action::Action;
use action::model::mean_magnetization::sample_mean_magnetization;
use action::model::observables::{Magnetization, SpinLattice};
use action::monte_carlo::metropolis_step::MonteCarloSimulation;

fn main() {
    let mut simulation = MonteCarloSimulation::new(569);

    const SIZE: usize = 5;
    let mut lattice = SpinLattice::<SIZE>::new();

    let mut magnetizations = Vec::<Magnetization>::default();

    for _sweep in 1..10 {
        for i in 1..SIZE {
            for j in 1..SIZE {
                let action = Action::local(
                    lattice.site(i as isize, j as isize),
                    lattice.neighborhood(i as isize, j as isize),
                );
                lattice.sites[i][j] = simulation.step(action).update(lattice.sites[i][j]);
            }
        }
        println!("\n{}, m = {:?}", lattice, lattice.magnetization());
        magnetizations.push(lattice.magnetization());
    }

    println!("{:?}", sample_mean_magnetization(magnetizations.iter()));
}
