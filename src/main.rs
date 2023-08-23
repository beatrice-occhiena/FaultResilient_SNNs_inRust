use group02::network::config::{build_network_from_setup, network_setup_from_file};
use group02::resilience::gui;

fn main() {

    // Possible idea for the file configuration implementation (INCOMPLETE)
    //******************************************************************
    //let _ = gui::launch();

    let n = network_setup_from_file();
    let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

    // Processing the input
    let mut vec_max = Vec::new();
    for input_spikes in input_spike_train {
        let output_spikes = snn.process_input(&input_spikes, None);
        let max = compute_max_output_spike(output_spikes);
        vec_max.push(max);
    }

    // Writing the results to output file
    let acc = compute_accuracy(vec_max, targets);
    println!("Accuracy = {}%", acc);

    // Possible idea for the GUI implementation
    //******************************************************************
    //while !gui::is_gui_closed() {
    // Launch the GUI to collect user input
    // instead of   let _ = gui::launch();
    //let selected_fault = gui::get_user_fault_selection();

    // OPTIONAL build the network based on user input from the GUI
    // if the user has overwritten the configuration file
    // (only if we decide to implement the configuration file on the GUI
    // otherwise we can ignore this part)

    // Process the input with faults injected based on user selection
    // ... multithread ...

    // Pass the results to the GUI for visualization
    //gui::visualize_results(results);
    // }
}

pub fn compute_accuracy(vec_max: Vec<u8>, targets: Vec<u8>) -> f64 {
    let matching = vec_max.iter().zip(&targets).filter(|&(a, b)| a == b).count();
    ((matching * 100) / targets.len()) as f64
}

pub fn compute_max_output_spike(output_spikes: Vec<Vec<u8>>) -> u8 {
    let mut vec_sum = Vec::new();
    for o in output_spikes {
        vec_sum.push(o.iter().sum());
    }
    let mut max = 0;
    let mut max_j = 0;
    for (j, v) in vec_sum.iter().enumerate() {
        if *v > max {
            max = *v;
            max_j = j;
        }
    }
    println!("max -> {}", max_j);
    max_j as u8
}