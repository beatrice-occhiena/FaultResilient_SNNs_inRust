#[allow(unused_imports)]
use group02::network::config::{build_network_from_setup, compute_accuracy, compute_max_output_spike, network_setup_from_file};
use group02::resilience::components::ComponentType;
use group02::resilience::fault_models::FaultType;
#[allow(unused_imports)]
use group02::resilience::gui;
use group02::resilience::simulation::UserSelection;

fn main() {

    // GUI TOOL implementation
    //******************************************************************

    //let _ = gui::launch();

    // CODE FOR TESTING PURPOSES
    //****************************************************************** 

    let n = network_setup_from_file();
    let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

    // INITIAL PROCESSING
    //*******************
    /*
    let mut vec_max = Vec::new();
    for input_spikes in input_spike_train.iter() {
        let output_spikes = snn.process_input(&input_spikes, None);
        let max = compute_max_output_spike(output_spikes);
        vec_max.push(max);
    }

    let acc = compute_accuracy(vec_max, &targets);
    println!("Accuracy = {}%", acc);*/

    let acc = 98.0;

    // SIMULATION
    //***********
    let us = UserSelection::new(vec![ComponentType::MembranePotential], FaultType::StuckAt1, 2, input_spike_train);
    let results = snn.run_simulation(us, targets, acc);

    for (acc, fault) in results {
        println!(""); // empty line
        println!("Injected fault info:");
        println!("{:?}", fault);
        println!("Resulting accuracy = {}%", acc);
        println!(""); // empty line
    }

}