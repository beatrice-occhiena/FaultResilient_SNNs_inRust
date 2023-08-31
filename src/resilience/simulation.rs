/* Defines the simulation logic to be used in the resilience analysis. */
use std::thread;
use std::thread::JoinHandle;
use rand::Rng;
use crate::network::config::{compute_accuracy, compute_max_output_spike};
use crate::network::neuron::neuron::Neuron;
use crate::network::layer::Layer;
use crate::network::snn::SNN;
use crate::resilience::components::ComponentType;
use crate::resilience::fault_models::{FaultType, InjectedFault, ApplyFault};

// Struct to hold the fault injection parameters defined by the user
#[derive(Debug, Clone)]
pub struct UserSelection {
    pub components: Vec<ComponentType>,
    pub fault_type: FaultType,
    pub num_faults: u64,
    pub input_sequence: Vec<Vec<Vec<u8>>>,
}

impl UserSelection {
    // Constructor
    pub fn new(components: Vec<ComponentType>, fault_type: FaultType, num_faults: u64, input_sequence: Vec<Vec<Vec<u8>>>) -> Self {
        UserSelection {
            components,
            fault_type,
            num_faults,
            input_sequence,
        }
    }
}

impl < N: Neuron + Clone + Send + 'static > SNN < N >
{
    /**
     * Given the user selection, run the simulation of the SNN with the injected faults.
     * @param user_selection: UserSelection object containing the fault injection parameters defined by the user.
     * @param targets: Vector of target values for the input sequence (used to compute the accuracy of the SNN with the injected faults).
     * @return Vector of tuples containing:
     *  - the accuracy of the SNN with the injected faults
     *  - all the information about the injected fault
     */
    pub fn run_simulation(&self, user_selection: UserSelection, targets: Vec<u8>, no_faults_accuracy: f64) -> Vec<(f64,InjectedFault)> {

        let mut thread_handles = Vec::<JoinHandle<(f64,InjectedFault)>>::new();
        let mut vec_results = Vec::<(f64,InjectedFault)>::new();

        // For each fault to be injected
        for _ in 0..user_selection.num_faults {

            // Clone the user selection and the SNN to be used in separate threads
            let user_selection = user_selection.clone();
            let snn = self.clone();
            let targets = targets.clone();

            // Spawn a thread to run the simulation
            let handle = thread::spawn(move || {
                // Input sequence
                let input_spikes = user_selection.input_sequence;
                let num_time_steps = input_spikes.get(0).unwrap().get(0).unwrap().len();

                let mut v = Vec::new();

                // Randomly generate the injected fault
                let mut injected_fault = Self::generate_random_fault(user_selection.components,user_selection.fault_type, &snn, &num_time_steps);
                let mut already_injected = false;

                // Apply the injected fault to the cloned SNN
                // - if the fault is a static fault
                // - AND the component selected doesn't change over time
                if injected_fault.fault_type != FaultType::TransientBitFlip 
                && injected_fault.component_type.is_static_component() {
                    
                    // Check if the applied fault actually modifies the value of the bit in the component
                    // - if the bit was 0 and the fault is stuck-at-0 => the fault doesn't need to be applied
                    // - if the bit was 1 and the fault is stuck-at-1 => the fault doesn't need to be applied
                    let bit_unchanged = snn.apply_fault_before_processing(&mut injected_fault);

                    if bit_unchanged {
                        // There's no need to run the simulation -> the result is the same as the original SNN
                        // => return the accuracy of the original SNN
                        // => exit the thread
                        return (no_faults_accuracy, injected_fault);
                    }else{
                        // The fault has been applied to the SNN
                        // => it doesn't need to be injected again during the processing phase
                        // => continue with the simulation
                        already_injected = true;
                    }
                }

                for input_spike_train in input_spikes {
                    
                    // Process the input sequence
                    let output_spikes;
                    if already_injected { // with the fault already injected
                        output_spikes = snn.process_input(&input_spike_train, None)
                    }else{ // with the fault to be injected during the processing phase
                        output_spikes = snn.process_input(&input_spike_train, Some(injected_fault));
                    };

                    // Compute accuracy
                    let max = compute_max_output_spike(output_spikes);
                    v.push(max);
                }
                
                let a = compute_accuracy(v, &targets);
                let injected_fault = injected_fault.clone();
                (a, injected_fault)
            });
            thread_handles.push(handle);
        }

        // wait for the threads to finish and collect the results
        // - accuracy
        // - injected fault info
        for handle in thread_handles {
            let result = handle.join().unwrap();
            vec_results.push(result);
        }

        vec_results
            
    }

    fn generate_random_fault(components: Vec<ComponentType>, fault_type: FaultType,snn: &SNN<N>, num_time_steps: &usize) -> InjectedFault {
                
        // If the fault is a transient bit-flip fault
        // -> Select a random time step from the input sequence
        let mut time_step: Option<u64> = None;
        if fault_type == FaultType::TransientBitFlip {
            time_step = Some(rand::thread_rng().gen_range(0..*num_time_steps) as u64);
        }

        // Select a random component from the list of components
        let component_index = rand::thread_rng().gen_range(0..components.len());
        let component_type = components[component_index];

        // Identify the category of the component
        let component_category = component_type.get_category();

        // Select a random layer from the list of layers
        let layer_index = rand::thread_rng().gen_range(0..snn.get_num_layers());

        // Select a random index of the component from the list of components of the given type in the layer
        let layer = snn.get_layer(layer_index);
        let num_components = layer.lock().unwrap().get_num_components_from_type(&component_type);
        let component_index = rand::thread_rng().gen_range(0..num_components);

        // Select a random bit index for the component (not for threshold comparators)
        let mut bit_index: Option<usize> = None;
        if component_type != ComponentType::ThresholdComparator {
            bit_index = Some(rand::thread_rng().gen_range(0..64));
        }

        // Create and return the injected fault object
        InjectedFault::new(fault_type, time_step, layer_index, component_type, component_category, component_index, bit_index)

    }

    /**
     * Apply the injected fault to the SNN before processing the input sequence.
     * @param injected_fault: information about the fault to be injected.
     * @return true if the bit in the component is unchanged after the fault is applied.
     */
    fn apply_fault_before_processing(&self, injected_fault: &mut InjectedFault) -> bool {
        
        // Select the component to be modified
        // 1 - access the layer
        // 2 - inject the fault
        // 3 - check if the bit in the component stays unchanged
        let layer = self.get_layer(injected_fault.layer_index);
        let mut layer = layer.lock().unwrap();
        let bit_unchanged = layer.apply_fault_in_component(injected_fault);

        bit_unchanged
    }

}

impl <N: Neuron+ Clone + Send + 'static> Layer<N> {

    fn apply_fault_in_component(&mut self, fault_info: &mut InjectedFault) -> bool{

        let i_weight = fault_info.component_index / self.get_extra_weights()[0].len();
        let j_weight = fault_info.component_index % self.get_extra_weights()[0].len();

        // Access the variable representing the component
        // 1 - save the reference to the component in a variable
        let component = match fault_info.component_type {
            ComponentType::Extra => &mut self.extra_weights[i_weight][j_weight],
            ComponentType::Intra => &mut self.intra_weights[i_weight][j_weight],
            _ => self.neurons[fault_info.component_index].get_parameter_to_fault(fault_info.component_type),
        };

        let bit_unchanged = false;
        let bit_index = fault_info.bit_index.unwrap();
        let var_in_bits = (*component).to_bits();

        //#to_do remove log
        println!("component before: {}", *component);

        // Inject the fault
        match fault_info.fault_type {
            FaultType::StuckAt0 => {
                if var_in_bits & (1 << bit_index) == 0 {

                    //#to_do remove log
                    println!("component unchanged");

                    return true; // The bit is already 0 -> the fault doesn't need to be applied
                }
                else {
                    *component = (*fault_info).apply_fault(*component, 0);
                }
            },
            FaultType::StuckAt1 => {
                if var_in_bits & (1 << bit_index) != 0 {

                    //#to_do remove log
                    println!("component unchanged");

                    return true; // The bit is already 1 -> the fault doesn't need to be applied
                }
                else {
                    *component = (*fault_info).apply_fault(*component, 0);
                }
            },
            _ => panic!("Only static faults can be injected before the processing phase."),
        }

        // #to_do remove log
        println!("component after: {}", *component);

        bit_unchanged
    }
}