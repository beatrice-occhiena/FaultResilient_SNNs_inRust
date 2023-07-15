use crate::network::neuron::neuron::Neuron;
use crate::network::event::spike_event::SpikeEvent;

#[derive(Debug)]
pub struct Layer<N> 
where N: Neuron + Clone
{
    neurons: Vec<N>,                // neurons in a layer
    extra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the same layer
    prev_output_spikes: Vec<u8>,    // output vector (0/1) in a layer at time instant t-1: used to decrease the v_mem according to the intra_weights
}

impl <N: Neuron + Clone> Layer<N> {

  pub fn new(
    neurons: Vec<N>,
    extra_weights: Vec<Vec<f64>>,
    intra_weights: Vec<Vec<f64>>
  ) -> Self {
    
    let num_n = neurons.len();
    let num_n_prev = extra_weights.len();
    let num_n_same = intra_weights.len();

    // check if the number of neurons in the previous layer is equal to the number of rows in extra_weights
    if num_n_prev != num_n {
      panic!("The number of neurons in the previous layer is not equal to the number of rows in extra_weights");
    }
    else if  {
        
    }
    Layer { 
      neurons, 
      extra_weights, 
      intra_weights, 
      prev_output_spikes: vec![0; num_n] 
    }

    // #to_do: possible check ???
    // the number of neurons in the previous layer must be equal to the number of rows in extra_weights
  }

  //Getters
  



}