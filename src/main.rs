mod simulation_mc;

use simulation_mc::monte_carlo::black_scholes_merton_mc;
use simulation_mc::visualization::{plot_final_distribution, plot_mc_paths, plot_statistics};
use statrs::statistics::Statistics;

fn main() {
    let S0 = 125.0;
    let M = 100.0;
    let T = 1.0; // 1 year
    let I = 1_000_000.0;
    let sigma = 0.17;
    let r = 0.23;
    let n_paths: usize = 100;

    monte_carlo_simulation(S0, M, T, I, sigma, r, n_paths);
}

fn monte_carlo_simulation(S0: f64, M: f64, T: f64, I: f64, sigma: f64, r: f64, n_paths: usize) {
    println!("Monte Carlo Simulation: Start");
    let paths = black_scholes_merton_mc(S0, M, T, I, sigma, r);
    println!("Monte Carlo Simulation: End");

    // First graphic:
    println!("Paths graphic generation");
    let paths_plot = plot_mc_paths(&paths, n_paths);
    paths_plot.show();

    // Second graph: Final distribution
    println!("Histogram of the final prices");
    let hist_plot = plot_final_distribution(&paths);
    hist_plot.show();

    // Third: Statistics plot
    println!("Statistics plot");
    let stats_plot = plot_statistics(&paths);
    stats_plot.show();

    // Final statistics
    let final_prices = paths.row(paths.shape()[0] - 1);
    let mean_final = final_prices.mean();
    let std_final = final_prices.std(1.0);
    let min_final = final_prices
        .iter()
        .fold(f64::INFINITY, |acc, x| acc.min(*x));
    let max_final = final_prices
        .iter()
        .fold(f64::NEG_INFINITY, |acc, x| acc.max(*x));

    println!("\n========FINAL STATISTICS==========\n");
    println!("Initial prices: {:.2}", S0);
    println!("Final prices: {:.2}", mean_final);
    println!("Minimum final prices: {:.2}", min_final);
    println!("Maximum final prices: {:.2}", max_final);
    println!("Mean final prices: {:.2}", mean_final);
    println!("Final standard deviation: {:.2}", std_final);
    println!(
        "Condidence 95%: [{:.2}, {:.2}]",
        mean_final - 1.96 * std_final,
        mean_final + 1.96 * std_final
    );
}
