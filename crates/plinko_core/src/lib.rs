pub mod board;
pub mod geometry;
pub mod physics;

pub use board::*;
pub use geometry::*;
pub use physics::*;

use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sim_core::{Evaluation, Seed, StochasticExperiment};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlinkoParams {
    // Value between 0.0 and 1.0 representing normalized horizontal drop position of the ball
    pub drop_x: f64,
    pub ball_radius: f64,
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
    pub board: Board,
    pub bin_count: usize,
    pub target_bin: usize,
    pub physics: PhysicsConfig,
    pub max_steps: usize,
}

impl PlinkoExperiment {
    pub fn new(bin_count: usize, target_bin: usize) -> Self {
        assert!(bin_count > 0, "bin_count must be positive");
        assert!(
            target_bin < bin_count,
            "target_bin must be less than bin_count"
        );

        Self {
            board: make_standard_board(10.0, 10.0, 5, 5, 0.5),
            bin_count,
            target_bin,
            physics: PhysicsConfig {
                gravity: Vec2::new(0.0, 9.8),
                dt: 0.01,
                restitution: 0.75,
            },
            max_steps: 1000,
        }
    }
}

impl StochasticExperiment for PlinkoExperiment {
    type Params = PlinkoParams;
    type Outcome = PlinkoOutcome;
    type Metrics = PlinkoMetrics;

    fn run_trial(&self, params: &Self::Params, seed: Seed) -> Self::Outcome {
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);
        let noise = rng.random_range(-0.05..0.05);
        let drop_x = (params.drop_x + noise).clamp(0.0, 1.0)
            * self
                .board
                .width
                .clamp(params.ball_radius, self.board.width - params.ball_radius);
        let initial_ball = Ball {
            position: Vec2::new(drop_x, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: params.ball_radius,
        };
        let ball_exit =
            run_ball_until_exit(&self.board, initial_ball, &self.physics, self.max_steps);
        let bin_index =
            bin_index_for_x(ball_exit.final_position.x, self.board.width, self.bin_count);
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
            ball_radius: 0.5,
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
            ball_radius: 0.5,
        };
        let seeds = sequential_seeds(100);

        let evaluation = experiment.evaluate(&params, &seeds);

        assert!((0.0..=1.0).contains(&evaluation.metrics.target_bin_probability));
    }
}
