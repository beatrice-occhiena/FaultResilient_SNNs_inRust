/* Defines the simulation logic to be used in the resilience analysis. */
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
