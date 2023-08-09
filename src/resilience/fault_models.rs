/* Module for fault models */

use rand::Rng; // Import random number generator
use crate::resilience::components::{ComponentType, ComponentCategory};


// Enum representing the different fault types
pub enum FaultType {
    StuckAt0,
    StuckAt1,
    TransientBitFlip,
}

// Struct representing a fault occurrence with its properties
struct InjectedFault {
    // FAULT PROPERTIES
    fault_type: FaultType,
    time_step: u64,                         // Time step at which the fault must be injected (for transient bit-flip faults only)
    // FAULT LOCATION
    layer_index: usize,                     // Layer index of the component in which the fault must be injected
    component_category: ComponentCategory,  // Category of component in which the fault must be injected
    component_type: ComponentType,          // Type of component in which the fault must be injected
    component_index: usize,                 // Index of the component in which the fault must be injected
    bit_index: usize,                       // Bit index of the component in which the fault must be injected
}

impl InjectedFault {
    // Constructor
    fn new(fault_type: FaultType, time_step: u64, layer_index: usize, component_type: ComponentType, component_category: ComponentCategory, component_index: usize, bit_index: usize) -> Self {
        InjectedFault {
            fault_type,
            time_step,
            layer_index,
            component_category,
            component_type,
            component_index,
            bit_index,
        }
    }
}