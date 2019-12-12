use preexplorer::prelude::*;

fn main() {
    comparing_interations();

    increasing_comparisons();
}

fn comparing_interations() {
    // Computing the data

    let data_1 = vec![0. as f32, 1., 2., 3., 4., 5.];
    let data_2 = vec![0., 1.4, 10., 4.];

    // Define plotables

    let seq_1 = data_1.preexplore();
    let seq_2 = data_2.preexplore();

    // Create comparison and plot

    pre::sequence::Comparison::new(vec![seq_1, seq_2])
        .title("All together")
        .plot("1")
        .unwrap();
}

fn increasing_comparisons() {
    // First Sequence

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let plotting_1 = data_1.preexplore().title("First").to_owned();

    // Another sequence

    let data_2 = vec![0., 1.4, 10., 4.];
    let group_of_plottings = vec![data_2.preexplore().title("Second").to_owned()];
    let mut comparison_plotting = plotting_1.compare_with(group_of_plottings);

    // Keep adding more

    let data_3 = vec![0.1, 1.5, 7., 5.];
    let group_of_plottings = vec![data_3.preexplore().title("Third").to_owned()];
    comparison_plotting.add(group_of_plottings);

    // Change some settings

    comparison_plotting.title("More comparisons");

    // Plot everything

    comparison_plotting.id("my_serie_name").plot("2").unwrap();
}
