/* Defines the simulation logic to be used in the resilience analysis. */
use rand::Rng; // Import random number generator
use crate::network::neuron::neuron::Neuron;
use crate::network::snn::SNN;
use crate::resilience::components::ComponentType;
use crate::resilience::fault_models::FaultType;

// Struct to hold the fault injection parameters defined by the user
struct UserSelection {
    components: Vec<ComponentType>,
    fault_type: FaultType,
    num_faults: u64,
    input_sequence: Vec<Vec<u8>>,
}

impl UserSelection {
    // Constructor
    fn new(components: Vec<ComponentType>, fault_type: FaultType, num_faults: u64, input_sequence: Vec<Vec<u8>>) -> Self {
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

    pub fn run_simulation(&mut self, user_selection: UserSelection) {

        // Input sequence
        let input_spikes = user_selection.input_sequence;
        let num_time_steps = input_spikes[0].len();

        // For each fault to be injected
        for _ in 0..user_selection.num_faults {

            // If the fault is a transient bit-flip fault
            // -> Select a random time step from the input sequence
            let mut time_step: Option<usize> = None;
            if user_selection.fault_type == FaultType::TransientBitFlip
            {
                time_step = Some(rand::thread_rng().gen_range(0..num_time_steps));
            }

            // Select a random component from the list of components
            let component_index = rand::thread_rng().gen_range(0..user_selection.components.len());
            let component_type = user_selection.components[component_index];

            // Identify the category of the component
            let component_category = component_type.get_category();

            // Select a random layer from the list of layers
            let layer_index = rand::thread_rng().gen_range(0..self.get_num_layers());

            // Select a random index of the component from the list of components of the given type in the layer
            let layer = self.get_layer(layer_index);
            let num_components = layer.lock().unwrap().get_num_components_from_type(&component_type);
            let component_index = rand::thread_rng().gen_range(0..num_components);

            

            
        }
    }

    
}

/*
pub struct InjectedFault {
    // FAULT PROPERTIES
    fault_type: FaultType,
    time_step: Option<u64>,                 // Time step at which the fault must be injected (for transient bit-flip faults only)
    // FAULT LOCATION
    layer_index: usize,                     // Layer index of the component in which the fault must be injected
    component_category: ComponentCategory,  // Category of component in which the fault must be injected
    component_type: ComponentType,          // Type of component in which the fault must be injected
    component_index: usize,                 // Index of the component in which the fault must be injected
    bit_index: Option<usize>,               // Bit index of the component in which the fault must be injected (not for threshold comparators)
}
 */