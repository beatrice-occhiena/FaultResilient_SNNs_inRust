/* Defines the simulation logic to be used in the resilience analysis. */
use std::thread;
use std::thread::JoinHandle;
use rand::Rng;
use crate::network::config::{compute_accuracy, compute_max_output_spike};
// Import random number generator
use crate::network::neuron::neuron::Neuron;
use crate::network::snn::SNN;
use crate::resilience::components::ComponentType;
use crate::resilience::fault_models::{FaultType, InjectedFault};

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
    pub fn run_simulation(&self, user_selection: UserSelection, targets: Vec<u8>) -> Vec<(f64,InjectedFault)> {

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
                let injected_fault = Self::generate_random_fault(user_selection.components,user_selection.fault_type, &snn, &num_time_steps);

                for input_spike_train in input_spikes {
                    // Process the input sequence with the injected fault
                    let output_spikes = snn.process_input(&input_spike_train, Some(injected_fault));
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

}
