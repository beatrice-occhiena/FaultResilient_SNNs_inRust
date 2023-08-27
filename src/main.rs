use group02::network::config::{build_network_from_setup, compute_accuracy, compute_max_output_spike, network_setup_from_file};
use group02::resilience::components::ComponentType;
use group02::resilience::fault_models::FaultType;
use group02::resilience::gui;
use group02::resilience::simulation::UserSelection;

fn main() {

    // Possible idea for the file configuration implementation (INCOMPLETE)
    //******************************************************************
    // let _ = gui::launch();

    // 
    let n = network_setup_from_file();
    let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

    // Processing the input
    let mut vec_max = Vec::new();
    for input_spikes in input_spike_train.iter() {
        let output_spikes = snn.process_input(&input_spikes, None);
        let max = compute_max_output_spike(output_spikes);
        vec_max.push(max);
    }

    let acc = compute_accuracy(vec_max, &targets);
    println!("Accuracy = {}%", acc);

    let us = UserSelection::new(vec![ComponentType::Extra], FaultType::StuckAt0, 10, input_spike_train);
    let results = snn.run_simulation(us, targets);
    
    for (acc, fault) in results {
        println!(""); // empty line
        println!("Injected fault info:");
        println!("{:?}", fault);
        println!("Resulting accuracy = {}%", acc);
        println!(""); // empty line
    }

    // Possible idea for the GUI implementation
    // ******************************************************************
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