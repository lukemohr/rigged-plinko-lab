use serde::{Deserialize, Serialize};

/// A seed used to make simulation runs reproducible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Seed(pub u64);

/// Result of evaluating one parameter set over many stochastic trials.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaluation<M> {
    pub trials: usize,
    pub metrics: M,
}

/// A stochastic experiment that can be run repeatedly with different random seeds.
pub trait StochasticExperiment {
    type Params;
    type Outcome;
    type Metrics;

    fn run_trial(&self, params: &Self::Params, seed: Seed) -> Self::Outcome;

    fn evaluate(&self, params: &Self::Params, seeds: &[Seed]) -> Evaluation<Self::Metrics>;
}
