/* Module for fault models */

use rand::Rng;

// Enum representing the different fault types
#[derive(Debug, Clone, Copy)]
enum FaultType {
    StuckAt0,
    StuckAt1,
    TransientBitFlip,
}

// Struct representing a fault occurrence with its properties
#[derive(Debug, Clone, Copy)]
struct Fault {
    component_index: usize,
    bit_index: usize,
    fault_type: FaultType,
}

// Function to generate a fault occurrence
fn generate_fault(num_components: usize, num_bits_per_component: usize, fault_type: FaultType) -> Fault {
    
    let mut rng = rand::thread_rng(); // Initialize random number generator
    let component_index = rng.gen_range(0..num_components);
    let bit_index = rng.gen_range(0..num_bits_per_component);

    Fault {
        component_index,
        bit_index,
        fault_type,
    }
}
