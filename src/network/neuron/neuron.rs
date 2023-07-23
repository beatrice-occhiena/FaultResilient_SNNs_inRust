// generic trait Neuron that defines the interface for a neuron
pub trait Neuron {
    fn process_input(&mut self, time: u64, weights_sum: f64) -> u8;
    fn initialize(&mut self);
}