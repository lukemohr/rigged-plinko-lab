pub mod geometry;
pub mod physics;

use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sim_core::{Evaluation, Seed, StochasticExperiment};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlinkoParams {
    /// Horizontal drop location, normalized so 0.0 is left and 1.0 is right.
    pub drop_x: f64,

    /// A temporary "bias" parameter.
    ///
    /// Later this will be replaced by actual peg geometry. For now, it lets us
    /// test the experiment/evaluation structure before writing physics.
    pub board_bias: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlinkoOutcome {
    pub bin_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlinkoMetrics {
    pub bin_counts: Vec<usize>,
    pub target_bin_probability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlinkoExperiment {
    pub bin_count: usize,
    pub target_bin: usize,
}

impl PlinkoExperiment {
    pub fn new(bin_count: usize, target_bin: usize) -> Self {
        assert!(bin_count > 0, "bin_count must be positive");
        assert!(
            target_bin < bin_count,
            "target_bin must be less than bin_count"
        );

        Self {
            bin_count,
            target_bin,
        }
    }
}

impl StochasticExperiment for PlinkoExperiment {
    type Params = PlinkoParams;
    type Outcome = PlinkoOutcome;
    type Metrics = PlinkoMetrics;

    fn run_trial(&self, params: &Self::Params, seed: Seed) -> Self::Outcome {
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);

        // Temporary toy model:
        // - start from drop_x
        // - add board_bias
        // - add random noise
        //
        // Later, this will become actual ball/peg simulation.
        let noise = rng.random_range(-0.35..0.35);
        let landing_x = (params.drop_x + params.board_bias + noise).clamp(0.0, 0.999_999);

        let bin_index = (landing_x * self.bin_count as f64).floor() as usize;

        PlinkoOutcome { bin_index }
    }

    fn evaluate(&self, params: &Self::Params, seeds: &[Seed]) -> Evaluation<Self::Metrics> {
        let mut bin_counts = vec![0; self.bin_count];

        for seed in seeds {
            let outcome = self.run_trial(params, *seed);
            bin_counts[outcome.bin_index] += 1;
        }

        let target_bin_probability = if seeds.is_empty() {
            0.0
        } else {
            bin_counts[self.target_bin] as f64 / seeds.len() as f64
        };

        Evaluation {
            trials: seeds.len(),
            metrics: PlinkoMetrics {
                bin_counts,
                target_bin_probability,
            },
        }
    }
}

pub fn sequential_seeds(n: usize) -> Vec<Seed> {
    (0..n).map(|i| Seed(i as u64)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluation_counts_all_trials() {
        let experiment = PlinkoExperiment::new(7, 3);
        let params = PlinkoParams {
            drop_x: 0.5,
            board_bias: 0.0,
        };
        let seeds = sequential_seeds(100);

        let evaluation = experiment.evaluate(&params, &seeds);

        assert_eq!(evaluation.trials, 100);
        assert_eq!(evaluation.metrics.bin_counts.iter().sum::<usize>(), 100);
    }

    #[test]
    fn target_probability_is_between_zero_and_one() {
        let experiment = PlinkoExperiment::new(7, 3);
        let params = PlinkoParams {
            drop_x: 0.5,
            board_bias: 0.0,
        };
        let seeds = sequential_seeds(100);

        let evaluation = experiment.evaluate(&params, &seeds);

        assert!((0.0..=1.0).contains(&evaluation.metrics.target_bin_probability));
    }
}
