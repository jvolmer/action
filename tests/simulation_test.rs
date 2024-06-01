use action::model::action::Action;
use action::model::observables::{Magnetization, SpinLattice};
use action::model::spin::Spin;
use action::monte_carlo::metropolis_step::{MetropolisStep, MonteCarloSimulation};

#[test]
fn flip_all_spins_in_lattice_once() {
    const SIZE: usize = 5;
    let mut lattice = SpinLattice::<SIZE>::new_with(Spin::Up);
    assert_eq!(lattice.magnetization(), Magnetization(25));

    for i in 0..SIZE {
        for j in 0..SIZE {
            lattice.sites[i][j] = MetropolisStep::accept().update(lattice.sites[i][j]);
        }
    }

    assert_eq!(lattice.magnetization(), Magnetization(-25));
    assert_eq!(lattice, SpinLattice::<SIZE>::new_with(Spin::Down));
}

#[test]
fn spin_update_often() {
    let mut simulation = MonteCarloSimulation::new(569);

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
