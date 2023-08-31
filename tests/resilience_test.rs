use std::arch::x86_64::_SIDD_POSITIVE_POLARITY;

use group02::network::config::{build_network_from_setup, compute_accuracy, compute_max_output_spike, network_setup_from_file};
use group02::resilience::components::{ComponentType, ComponentCategory};
use group02::resilience::fault_models::{FaultType, InjectedFault};

#[test]
fn test_static_weight_fault_injection() {

    let n = network_setup_from_file();
    let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

    // component value pre fault injection
    let prev = snn.get_layer(1).lock().unwrap().extra_weights[0][0];

    // MANUAL FAULT INJECTION
    //***************************************************************************
    let fault_type: FaultType = FaultType::StuckAt0;
    let time_step: Option<u64> = None;
    let layer_index: usize = 1;
    let component_category: ComponentCategory = ComponentCategory::Connection;
    let component_type: ComponentType = ComponentType::Extra;
    let component_index: usize = 0;
    let bit_index: Option<usize> = Some(63);
    //***************************************************************************
    let fault = InjectedFault::new(fault_type, time_step, layer_index, component_type, component_category, component_index, bit_index);

    // PROCESSING WITH FAULT INJECTION
    let mut vec_max = Vec::new();
    for input_spikes in input_spike_train.iter() {
        let output_spikes = snn.process_input(&input_spikes, Some(fault));
        let max = compute_max_output_spike(output_spikes);
        vec_max.push(max);
    }
    let acc = compute_accuracy(vec_max, &targets);
    println!("Accuracy = {}%", acc);

    // component value post fault injection
    let post = snn.get_layer(1).lock().unwrap().extra_weights[0][0];

    // PRINT RESULTS
    println!(""); // empty line
    println!("Injected fault info:");
    println!("{:?}", fault);
    println!("Resulting accuracy = {}%", acc);
    println!(""); // empty line
    
    println!("value pre fault: {:?}", prev);
    println!("value post fault: {:?}", post);


}