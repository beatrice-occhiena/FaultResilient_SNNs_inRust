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


}