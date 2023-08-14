/* Module for fault models */
use crate::resilience::components::{ComponentType, ComponentCategory};

// Enum representing the different fault types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultType {
    StuckAt0,
    StuckAt1,
    TransientBitFlip,
}

impl FaultType {
    pub fn all() -> [FaultType; 3] {
        [FaultType::StuckAt0, FaultType::StuckAt1,FaultType::TransientBitFlip]
    }
}

// Struct representing a fault occurrence with its properties
#[derive(Debug, Clone, Copy)]
pub struct InjectedFault {
    // FAULT PROPERTIES
    pub fault_type: FaultType,
    pub time_step: Option<u64>,               // Time step at which the fault must be injected (for transient bit-flip faults only)
    // FAULT LOCATION
    pub layer_index: usize,                     // Layer index of the component in which the fault must be injected
    pub component_category: ComponentCategory,  // Category of component in which the fault must be injected
    pub component_type: ComponentType,          // Type of component in which the fault must be injected
    pub component_index: usize,                 // Index of the component in which the fault must be injected
    pub bit_index: Option<usize>,               // Bit index of the component in which the fault must be injected (not for threshold comparators)
}

impl InjectedFault {
    // Constructor
    pub fn new(fault_type: FaultType, time_step: Option<u64>, layer_index: usize, component_type: ComponentType, component_category: ComponentCategory, component_index: usize, bit_index: Option<usize>) -> Self {
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

    pub fn apply_fault_f64(&self, mut var: f64, timestamp: u64) -> f64 {

        // Convert f64 to u64 to get access to its representation in bits
        let mut var_in_bits = var.to_bits();

        if self.fault_type == FaultType::StuckAt0 {
            var_in_bits = Self::stuck_at_0(var_in_bits, self.bit_index.unwrap());
        }
        else if self.fault_type == FaultType::StuckAt1 {
            var_in_bits = Self::stuck_at_1(var_in_bits, self.bit_index.unwrap());
        }
        else if self.fault_type == FaultType::TransientBitFlip {
            if self.time_step.unwrap() == timestamp {
                var_in_bits = Self::bit_flip(var_in_bits, self.bit_index.unwrap());
            }
        }
        
        // Convert back to f64
        f64::from_bits(var_in_bits)
    }

    pub fn apply_fault_u64(&self, mut var: u64, timestamp: u64) -> u64 {
        if self.fault_type == FaultType::StuckAt0 {
            var = Self::stuck_at_0(var, self.bit_index.unwrap());
        }
        else if self.fault_type == FaultType::StuckAt1 {
            var = Self::stuck_at_1(var, self.bit_index.unwrap());
        }
        else if self.fault_type == FaultType::TransientBitFlip {
            if self.time_step.unwrap() == timestamp {
                var = Self::bit_flip(var, self.bit_index.unwrap());
            }
        }
        var
    }

    pub fn stuck_at_0(var: u64, bit_index: usize) -> u64 {
        var & !(1 << bit_index)
    }
    pub fn stuck_at_1(var: u64, bit_index: usize) -> u64 {
        var | (1 << bit_index)
    }
    pub fn bit_flip(var: u64, bit_index: usize) -> u64 {
        var ^ (1 << bit_index)
    }

}