use super::metropolis_step::MetropolisStep;

pub struct BoltzmannWeight {
    weight: f64,
}
impl BoltzmannWeight {
    pub fn new(weight: f64) -> Self {
        Self { weight }
    }
    pub fn step(&self, rand: f64) -> MetropolisStep {
        MetropolisStep::new(self.weight >= rand)
    }
}
