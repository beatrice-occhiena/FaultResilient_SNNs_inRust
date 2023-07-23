use std::sync::{Arc, Mutex};
use crate::network::layer::Layer;
use crate::network::neuron::neuron::Neuron;

#[derive(Debug)]
pub struct SNN < N: Neuron + Clone + Send + 'static,
                const InputSize: usize,
                const OutputSize: usize >
{
  layers:  Vec<Arc<Mutex<Layer<N>>>>, //InputSize, OutputSize ???
}

impl < N: Neuron + Clone + Send + 'static,
      const InputSize: usize,
      const OutputSize: usize >
SNN < N, InputSize, OutputSize >
{
  pub fn new(layers:  Vec<Arc<Mutex<Layer<N>>>>) -> Self {
    SNN { layers }
  }

  // Getters
  pub fn get_num_layers(&self) -> usize {
    self.layers.len()
  }

  pub fn get_layers(&self) -> &Vec<Arc<Mutex<Layer<N>>>> {
    &self.layers
  }

  /**
    It processes input spikes and produces the resulting output spikes
      => final inference

    @param spikes (Vec<Vec<u8>>)
    The input of the SNN is a matrix of 0/1, where each row represents the array of spikes received by a neuron.
    Each column represents the value of the input at a given instant of time
        => the number of columns represents the duration of the input
        => we have to set a quantization of the time considering:
            - a counter that increases at each time step (saved in each SpikeEvent struct)
            - the duration of the time step
            
    @return ???

   */
  pub fn process_input



}