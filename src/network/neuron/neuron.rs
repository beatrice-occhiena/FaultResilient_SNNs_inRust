use crate::resilience::fault_models::InjectedFault;
use crate::resilience::components::ComponentType;

// generic trait Neuron that defines the interface for a neuron
pub trait Neuron {
    fn process_input(&mut self, time: u64, weighted_sum: f64, fault: Option<InjectedFault>) -> u8;
    fn initialize(&mut self);
    fn get_parameter_to_fault(&mut self, component_type: ComponentType) -> &mut f64;
}