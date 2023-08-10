/* Module for fault models */
use crate::resilience::components::{ComponentType, ComponentCategory};

// Enum representing the different fault types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultType {
    StuckAt0,
    StuckAt1,
    TransientBitFlip,
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
    pub fn apply_fault(&self, mut weight: f64, timestamp: u64) -> f64 {
        if self.fault_type == FaultType::StuckAt0 {
            weight = Self::stuck_at_0_fault(weight, self.bit_index.unwrap());
        }
        if self.fault_type == FaultType::StuckAt1 {
            weight = Self::stuck_at_1_fault(weight, self.bit_index.unwrap());
        }
        if self.fault_type == FaultType::TransientBitFlip {
            if self.time_step.unwrap() == timestamp {
                weight = Self::bit_flip_fault(weight, self.bit_index.unwrap());
            }
        }
        weight
    }
    pub fn stuck_at_0_fault(var: f64, bit_index: usize) -> f64 {
        (var.to_bits() & !(1 << bit_index)) as f64
    }
    pub fn stuck_at_1_fault(var: f64, bit_index: usize) -> f64 {
        (var.to_bits() | (1 << bit_index)) as f64
    }
    pub fn bit_flip_fault(var: f64, bit_index: usize) -> f64 {
        (var.to_bits() ^ (1 << bit_index)) as f64
    }
}