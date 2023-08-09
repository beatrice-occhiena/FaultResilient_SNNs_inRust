/* Module for fault models */

use rand::Rng;
use crate::resilience::components::ComponentType;

// Enum representing the different fault types
#[derive(Debug, Clone, Copy)]
pub enum FaultType {
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

// Struct representing a fault model
#[derive(Debug, Clone)]
pub struct FaultModel {
    components: Vec<ComponentType>,
    fault_type: FaultType,
    num_faults: usize,
}


fn stuck_at_0_fault<T: std::ops::BitAnd<Output = T> + std::ops::Not<Output = T> + From<u8> + std::ops::Shl<usize, Output = T>>(var: T) -> T {
    let num_bits = std::mem::size_of::<T>() * 8;
    let mut rng = rand::thread_rng();
    let bit_to_stuck = rng.gen_range(0..num_bits);
    var & !(T::from(1u8) << bit_to_stuck)
}

fn stuck_at_1_fault<T: std::ops::BitOr<Output = T> + From<u8> + std::ops::Shl<usize, Output = T>>(var: T) -> T {
    let num_bits = std::mem::size_of::<T>() * 8;
    let mut rng = rand::thread_rng();
    let bit_to_stuck = rng.gen_range(0..num_bits);
    var | (T::from(1u8) << bit_to_stuck)
}
