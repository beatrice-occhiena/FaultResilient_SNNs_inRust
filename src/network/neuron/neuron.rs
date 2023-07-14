// generic trait Neuron that defines the interface for a neuron
pub trait Neuron {
    fn compute_v_mem(&mut self, time: u64, weights_sum: f64) -> u8;
    fn configure(&mut self);
}