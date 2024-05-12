use action::model::action::Action;
use action::model::observables::SpinLattice;
use action::monte_carlo::metropolis_step::MonteCarloSimulation;

fn main() {
    let mut simulation = MonteCarloSimulation::new(569);

    const SIZE: usize = 5;
    let mut lattice = SpinLattice::<SIZE>::new();

    for i in 1..SIZE {
        for j in 1..SIZE {
            let action = Action::local(
                lattice.site(i as isize, j as isize),
                lattice.neighborhood(i as isize, j as isize),
            );
            lattice.sites[i][j] = simulation.step(action).update(lattice.sites[i][j]);
        }
    }
}
