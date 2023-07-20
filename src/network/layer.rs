use crate::network::neuron::neuron::Neuron;
use crate::network::event::spike_event::SpikeEvent;

#[derive(Debug)]
pub struct Layer<N> 
where N: Neuron + Clone + Send + 'static
{
    neurons: Vec<N>,                // neurons in a layer
    extra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the same layer
    prev_output_spikes: Vec<u8>,    // output vector (0/1) in a layer at time instant t-1: used to decrease the v_mem according to the intra_weights
}

impl <N: Neuron + Clone + Send + 'static> Layer<N> {

  pub fn new(
    neurons: Vec<N>,
    extra_weights: Vec<Vec<f64>>,
    intra_weights: Vec<Vec<f64>>
  ) -> Self {
    
    let num_n = neurons.len();
    let num_n_prev = extra_weights.len();
    let num_n_same = intra_weights.len();

    // check the number of neurons for the layer is consistent with the number of rows in the weights matrices
    if num_n_prev != num_n || num_n_same != num_n 
    {
      panic!("The number of neurons in the layer is not consistent with the number of rows in the weights matrices.");
    }

    Layer { 
      neurons, 
      extra_weights, 
      intra_weights, 
      prev_output_spikes: vec![0; num_n] 
    }
  }

  // Getters
  pub fn get_num_neurons(&self) -> usize {
    self.neurons.len()
  }

  pub fn get_neurons(&self) -> &Vec<N> {
    &self.neurons
  }

  pub fn get_extra_weights(&self) -> &Vec<Vec<f64>> {
    &self.extra_weights
  }

  pub fn get_intra_weights(&self) -> &Vec<Vec<f64>> {
    &self.intra_weights
  }



}