use ndarray::prelude::*;
use ndarray::{Array1, Array2};
use plotly::common::Mode;
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};

pub fn plot_mc_paths(paths: &Array2<f64>, n_paths_to_show: usize) -> Plot {
    let M = paths.shape()[0] - 1;
    let time_steps = Array1::linspace(0.0, 1.0, M + 1);

    let mut plot = Plot::new();

    // Add only a subset of the paths for visibility
    for i in 0..n_paths_to_show.min(paths.shape()[1]) {
        let path = paths.column(i).to_vec();
        let trace = Scatter::new(time_steps.to_vec(), path)
            .mode(Mode::Lines)
            .name(&format!("Path {}", i + 1))
            .opacity(0.7);
        plot.add_trace(trace);
    }

    // Add the mean of path
    let mean_path: Vec<f64> = (0..=M).map(|t| paths.row(t).mean().unwrap()).collect();

    let mean_trace = Scatter::new(time_steps.to_vec(), mean_path)
        .mode(Mode::Lines)
        .name("Mean")
        .line(plotly::common::Line::new().width(3.0).color("red"));

    plot.add_trace(mean_trace);

    // Personnalize

    let layout = Layout::new()
        .title("Monte Carlo Simulation - Black-Scholes-Merton model")
        .x_axis(Axis::new().title("Time (Years)"))
        .y_axis(Axis::new().title("Stock price"))
        .show_legend(true);

    plot.set_layout(layout);

    plot
}

pub fn plot_final_distribution(paths: &Array2<f64>) -> Plot {
    let final_prices = paths.row(paths.shape()[0] - 1).to_vec();

    let trace = plotly::Histogram::new(final_prices)
        .name("Distribution of the final prices")
        .n_bins_x(50);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("Final distribution of prices")
        .x_axis(Axis::new().title("Final price"))
        .y_axis(Axis::new().title("Frequency"));

    plot.set_layout(layout);

    plot
}

pub fn plot_statistics(paths: &Array2<f64>) -> Plot {
    let M = paths.shape()[0] - 1;
    let time_steps = Array1::linspace(0.0, 1.0, M + 1);

    // Statistics computation
    let means: Vec<f64> = (0..=M).map(|t| paths.row(t).mean().unwrap()).collect();

    let std_devs: Vec<f64> = (0..=M).map(|t| paths.row(t).std(1.0)).collect();

    let upper_bound: Vec<f64> = means
        .iter()
        .zip(&std_devs)
        .map(|(mean, std)| mean + 1.96 * std)
        .collect();

    let lower_bound: Vec<f64> = means
        .iter()
        .zip(&std_devs)
        .map(|(mean, std)| mean - 1.96 * std)
        .collect();

    let mut plot = Plot::new();

    // Confidence range
    let upper_trace = Scatter::new(time_steps.to_vec(), upper_bound)
        .mode(Mode::Lines)
        .name("IC 95% sup")
        .line(plotly::common::Line::new().width(1.0).color("lightgray"));

    let lower_trace = Scatter::new(time_steps.to_vec(), lower_bound)
        .mode(Mode::Lines)
        .name("IC 95% inf")
        .line(plotly::common::Line::new().width(1.0).color("lightgray"));

    // Mean
    let mean_trace = Scatter::new(time_steps.to_vec(), means)
        .mode(Mode::Lines)
        .name("mean")
        .line(plotly::common::Line::new().width(3.0).color("blue"));

    plot.add_trace(upper_trace);
    plot.add_trace(lower_trace);
    plot.add_trace(mean_trace);

    let layout = Layout::new()
        .title("Statistics of simulation")
        .x_axis(Axis::new().title("Time (Years)"))
        .y_axis(Axis::new().title("Prices"))
        .show_legend(true);

    plot.set_layout(layout);

    plot
}
