use group02::network::config::{build_network_from_setup, compute_accuracy, compute_max_output_spike, network_setup_from_file};
use group02::resilience::components::{ComponentType, ComponentCategory};
use group02::resilience::fault_models::{FaultType, InjectedFault};

/* THE RESULT OF THESE TESTS REFERS TO THE ORIGINAL IMPORTED NETWORK */

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
This test injects a fault in the quantization parameter dt of the second neuron of the second layer.
- neuron corresponding to the digit 1
- bit at index 62 from 0 to 1
- threshold from 1.0 to infinity

Increasing dt above infinity we expect the neuron to **never fire**
- the digit 1 will never be recognized
- resulting accuracy = 84.0% */
#[test]
fn test_positive_dt_fault_injection() {

    let n = network_setup_from_file();
    let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

    // MANUAL FAULT INJECTION
    //***************************************************************************
    let fault_type: FaultType = FaultType::StuckAt1;
    let time_step: Option<u64> = None;
    let layer_index: usize = 1;
    let component_category: ComponentCategory = ComponentCategory::MemoryArea;
    let component_type: ComponentType = ComponentType::DT;
    let component_index: usize = 1;
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

/**
    This test injects a fault in the intra weight of the second input of the first neuron of the second layer.
    - neuron corresponding to the digit 0
    - input corresponding to the digit 2
    - bit at index 62 from 0 to 1
    - intra weight from 0.0 to 2.0

    Setting the intra weight to a positive value, we expect to introduce a **very slight inhibitory effect** on the neuron.
    However, since the input involved is just one out of 10, we expect a very small effect on the neuron's behaviour.
    In particular, whenever the digit 2 neuron fires, the digit 0 neuron membrane potential will be slightly reduced.
    - the digit 0 might be recognized less often than it should
    - resulting accuracy = 97.0%
 */
#[test]
fn test_intra_weight_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::Connection;
  let component_type: ComponentType = ComponentType::Intra;
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
    This test injects a fault in the extra weight of the first input of the first neuron of the second layer.
    - neuron corresponding to the digit 0
    - input corresponding to the first neuron of the first layer
    - bit at index 63 from 1 to 0
    - inter weight from -0.10384419 to 0.10384419

    Setting the extra weight to a positive value, we expect to introduce a **very very slight bias** on the neuron.
    However, since the input involved is just one out of 128, we expect a very small effect on the neuron's behaviour.
    In particular, whenever the first neuron of the previous layer fires, the digit 0 neuron membrane potential will be slightly augmented.
    - the digit 0 might be recognized more often than it should
    - resulting accuracy = 98.0%
 */
#[test]
fn test_extra_weight_fault_injection() {

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
    This test injects a fault in the time step (u64) of the first neuron of the second layer.
    - neuron corresponding to the digit 0
    - bit at index 0 from 0 to 1
    - time step from 0 to 2

    Setting the time step to a value greater than the actual processing time, will result in a panic due to
    an 'attempt to subtract with overflow' in the function 'process_input' of the neuron.rs file. In fact,
    the time step is a u64 and can't support negative values.
    - the program will panic
*/
#[test]
#[should_panic]
fn test_positive_ts_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::MemoryArea;
  let component_type: ComponentType = ComponentType::Ts;
  let component_index: usize = 2;
  let bit_index: Option<usize> = Some(0);
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
