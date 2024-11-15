use std::fmt::{Debug, Display, Formatter};

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
pub struct InjectedFault{
    // FAULT PROPERTIES
    pub fault_type: FaultType,                  // Type of fault
    pub time_step: Option<u64>,                 // Time step at which the fault must be injected (for transient bit-flip faults only)
    // FAULT LOCATION
    pub layer_index: usize,                     // Layer index of the component in which the fault must be injected
    pub component_category: ComponentCategory,  // Category of component in which the fault must be injected
    pub component_type: ComponentType,          // Type of component in which the fault must be injected
    pub component_index: usize,                 // Index of the component in which the fault must be injected
    pub bit_index: Option<usize>,               // Bit index of the component in which the fault must be injected (not for threshold comparators)
}

impl Display for InjectedFault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fault type: {}\n", String::from(self.fault_type))?;
        if self.time_step.is_some() {
            write!(f, "Time step: {}\n", self.time_step.unwrap())?;
        }
        write!(f, "Layer index: {}\n", self.layer_index)?;
        write!(f, "Component category: {}\n", String::from(self.component_category))?;
        write!(f, "Component type: {}\n", String::from(self.component_type))?;
        write!(f, "Component index: {}\n", self.component_index)?;
        if self.bit_index.is_some() {
            write!(f, "Bit index: {}\n", self.bit_index.unwrap())?;
        }
        write!(f, "")
    }
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

pub trait ApplyFault<T> {
    fn apply_fault(&self, var: T, timestamp: u64) -> T;
}

impl ApplyFault<f64> for InjectedFault {
    fn apply_fault(&self, var: f64, timestamp: u64) -> f64 {

        // #to_do print
        //println!("{:?}", var);

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
        let new_var = f64::from_bits(var_in_bits);

        // #to_do print
        //println!("{:?}", new_var);

        new_var
    }
}

impl ApplyFault<u64> for InjectedFault {
    fn apply_fault(&self, mut var: u64, timestamp: u64) -> u64 {
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
}
    
impl ApplyFault<u8> for InjectedFault {
    // This function simulate the effect of a fault on a variable of dimension 1 bit
    fn apply_fault(&self, spike_value: u8, timestamp: u64) -> u8 {
        
        if self.fault_type == FaultType::StuckAt0 {
            0
        }
        else if self.fault_type == FaultType::StuckAt1 {
            1
        }
        else if self.fault_type == FaultType::TransientBitFlip {
            if self.time_step.unwrap() == timestamp {
                1 - spike_value
            }
            else {
                spike_value
            }
        }
        else {
            spike_value
        }
    }
}