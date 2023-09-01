use group02::network::config::{build_network_from_setup, compute_accuracy, compute_max_output_spike, network_setup_from_file};
use group02::resilience::components::{ComponentType, ComponentCategory};
use group02::resilience::fault_models::{FaultType, InjectedFault};

/* THE RESULT OF THESE TESTS REFERS TO THE ORIGINAL IMPORTED NETWORK */

#[test]
fn test_weight_fault_injection() {

    let n = network_setup_from_file();
    let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

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

    // PRINT RESULTS
    println!(""); // empty line
    println!("Injected fault info:");
    println!("{:?}", fault);
    println!("Resulting accuracy = {}%", acc);
    println!(""); // empty line

}


/**
    This test injects a fault in the threshold of the first neuron of the second layer.
    - neuron corresponding to the digit 0
    - bit at index 63 from 0 to 1
    - threshold from 1.0 to -1.0

    Lowering the threshold below 0 we expect the neuron to **fire more often** than it should
    - more false positives for the digit 0
    - resulting accuracy = 95.0%
 */

#[test]
fn test_negative_threshold_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::MemoryArea;
  let component_type: ComponentType = ComponentType::Threshold;
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

  // PRINT RESULTS
  println!(""); // empty line
  println!("Injected fault info:");
  println!("{:?}", fault);
  println!("Resulting accuracy = {}%", acc);
  println!(""); // empty line

}

/**
    This test injects a fault in the threshold of the third neuron of the second layer.
    - neuron corresponding to the digit 2
    - bit at index 62 from 0 to 1
    - threshold from 1.0 to infinity

    Increasing the threshold above infinity we expect the neuron to **never fire**
    - the digit 2 will never be recognized
    - resulting accuracy = 89.0% */
#[test]
fn test_positive_threshold_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::MemoryArea;
  let component_type: ComponentType = ComponentType::Threshold;
  let component_index: usize = 2;
  let bit_index: Option<usize> = Some(62);
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

  // PRINT RESULTS
  println!(""); // empty line
  println!("Injected fault info:");
  println!("{:?}", fault);
  println!("Resulting accuracy = {}%", acc);
  println!(""); // empty line

}

/**
    This test injects a fault in the membrane potential of the third neuron of the second layer.
    - neuron corresponding to the digit 2
    - bit at index 63 from 0 to 1
    - membrane potential from x to -x

    Setting the membrane potential to a value below 0 we expect the neuron to **never fire**, since the threshold is 1.0
    - the digit 2 will never be recognized
    - resulting accuracy = 89.0%
 */
#[test]
fn test_negative_membrane_potential_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::MemoryArea;
  let component_type: ComponentType = ComponentType::MembranePotential;
  let component_index: usize = 2;
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

  // PRINT RESULTS
  println!(""); // empty line
  println!("Injected fault info:");
  println!("{:?}", fault);
  println!("Resulting accuracy = {}%", acc);
  println!(""); // empty line

}

/**
    This test injects a fault in threshold comparator of the first neuron of the second layer.
    - neuron corresponding to the digit 0
    - bool resulting from the comparison always true

    Setting the threshold comparator to return always true we expect the neuron to **always fire**
    - the digit 0 will always be recognized
    - resulting accuracy = 7.0%
 */
#[test]
fn test_positive_threshold_comparator_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::InternalProcessingBlock;
  let component_type: ComponentType = ComponentType::ThresholdComparator;
  let component_index: usize = 0;
  let bit_index: Option<usize> = None;
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

  // PRINT RESULTS
  println!(""); // empty line
  println!("Injected fault info:");
  println!("{:?}", fault);
  println!("Resulting accuracy = {}%", acc);
  println!(""); // empty line

}