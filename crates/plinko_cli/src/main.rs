use anyhow::Result;
use plinko_core::{sequential_seeds, PlinkoExperiment, PlinkoParams};
use sim_core::StochasticExperiment;

fn main() -> Result<()> {
    let experiment = PlinkoExperiment::new(7, 3);

    let params = PlinkoParams {
        drop_x: 0.5,
        board_bias: 0.0,
    };

    let seeds = sequential_seeds(1_000);
    let evaluation = experiment.evaluate(&params, &seeds);

    println!("Trials: {}", evaluation.trials);
    println!("Bin counts: {:?}", evaluation.metrics.bin_counts);
    println!(
        "Target bin probability: {:.3}",
        evaluation.metrics.target_bin_probability
    );

    Ok(())
}
