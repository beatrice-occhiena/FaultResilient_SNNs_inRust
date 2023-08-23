use std::fs::File;
use std::io::{Write};
use group02::network::config::{build_network_from_setup, network_setup_from_file};
use group02::resilience::gui;

fn main() {

    // Possible idea for the file configuration implementation (INCOMPLETE)
    //******************************************************************
    let n = network_setup_from_file();
    let (snn,input_spike_train)  = build_network_from_setup(n.unwrap());

    // Processing the input
    let mut vec_max = Vec::new();
    for (i,input_spikes) in input_spike_train.iter().enumerate() {
        let output_spikes = snn.process_input(&input_spikes, None);
        let mut vec_sum = Vec::new();
        for o in output_spikes {
            vec_sum.push(o.iter().sum());
        }
        let mut max = 0;
        let mut max_j = 0;
        for (j,v) in vec_sum.iter().enumerate() {
            if *v > max {
                max = *v;
                max_j = j;
            }
        }
        println!("max-{} -> {}", i, max_j);
        vec_max.push(max_j);
    }
    // Writing the results to output file
    write_to_output_file("./output.txt", vec_max);

    //let _ = gui::launch();
    //******************************************************************

    /*
    /* Building of a network with an input layer of 784 neurons, an hidden layer of 128 neurons and an output layer of 10 neurons */
    let input_length = 784;
    let hidden_length = 128;
    let output_length = 10;
    let spike_length = 100;
    let batch_size = 256;

    // Building neurons
    let hidden_neurons : Vec<Lif> = get_neurons(hidden_length);
    let output_neurons : Vec<Lif> = get_neurons(output_length);
    // Getting extra_weights from files
    let extra_weights1 : Vec<Vec<f64>> = get_extra_weights("./parameters/weightsFile1.txt", input_length, hidden_length);
    let extra_weights2 : Vec<Vec<f64>> = get_extra_weights("./parameters/weightsFile2.txt",hidden_length, output_length);
    // Building intra_weights
    let intra_weights1 : Vec<Vec<f64>> = get_intra_weights(hidden_length);
    let intra_weights2 : Vec<Vec<f64>> = get_intra_weights(output_length);

    //Building the SNN
    let snn = SNNBuilder::new(input_length)
        .add_layer(hidden_neurons, extra_weights1, intra_weights1)
        .add_layer(output_neurons, extra_weights2, intra_weights2)
        .build();

    // Getting input spike trains from file
    let input_spike_train : Vec<Vec<Vec<u8>>> = get_input_spike_train("./inputSpikes.txt", input_length, spike_length, batch_size);

    //let injected_fault = InjectedFault::new(FaultType::StuckAt1, None, 0, ComponentType::ThresholdComparator, ComponentCategory::MemoryArea, 0, None);
    let mut vec_max = Vec::new();
    // Processing the input
    for (i,input_spikes) in input_spike_train.iter().enumerate() {
        let output_spikes = snn.process_input(&input_spikes, None);
        let mut vec_sum = Vec::new();
        for o in output_spikes {
            vec_sum.push(o.iter().sum());
        }
        let mut max = 0;
        let mut max_j = 0;
        for (j,v) in vec_sum.iter().enumerate() {
            if *v > max {
                max = *v;
                max_j = j;
            }
        }
        println!("max-{} -> {}", i, max_j);
        vec_max.push(max_j);
    }
    // Writing the results to output file
    write_to_output_file("./output.txt", vec_max);
*/

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
    //******************************************************************

}



fn write_to_output_file(filename: &str, max_j: Vec<usize>) {
    // Creating or opening the file
    let mut output_file = File::create(filename).expect("Error: something went wrong creating the file");
    // Writing in the file
    for i in 0..max_j.len() {
        if i == max_j.len() - 1 {
            output_file.write_all(format!("{}", max_j[i]).as_bytes()).expect("Something went wrong writing in the file");
        }
        else {
            output_file.write_all(format!("{}, ", max_j[i]).as_bytes()).expect("Something went wrong writing in the file");
        }
    }
}