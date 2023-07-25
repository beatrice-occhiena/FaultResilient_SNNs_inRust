use core::num;
use std::sync::{Arc, Mutex};
use crate::network::layer::Layer;
use crate::network::neuron::neuron::Neuron;
use crate::network::event::spike_event::SpikeEvent;

use super::event::spike_event;

#[derive(Debug)]
pub struct SNN < N: Neuron + Clone + Send + 'static >
{
  layers:  Vec<Arc<Mutex<Layer<N>>>>,
}

impl < N: Neuron + Clone + Send + 'static > SNN < N >
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

  fn get_input_layer_num_neurons(&self) -> usize {
    self.layers[0].lock().unwrap().get_num_neurons()
  }

  fn get_output_layer_num_neurons(&self) -> usize {
    self.layers[self.layers.len()-1].lock().unwrap().get_num_neurons()
  }

  // #to_do: complete here

  /**
    It processes input spikes and produces the resulting output spikes
      => final inference

    @param spikes (&Vec<Vec<u8>>)
    The input of the SNN is a matrix of 0/1, where each row represents the array of spikes received by each input neuron.
    Columns represent the value of the input at a given instant of time
        => the number of columns represents the duration of the input
        => we have to set a quantization of the time considering:
            - a counter that increases at each time step (saved in each SpikeEvent struct)
            - the dt duration of the time step

    @return Vec<Vec<u8>>
    The output of the SNN is a matrix of 0/1, where each row represents the array of spikes produced by each output neuron.
   */
  pub fn process_input(&self, spikes: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    // convert the input spikes into spike events
    let input_spike_events = self.derive_input_spike_events(spikes);

    // process the input spike events
    let output_spike_events = self.process_input_spike_events(&input_spike_events);

    // convert the output spike events into output spikes
    let output_spikes = self.derive_output_spikes(&output_spike_events);

    output_spikes   
  }

  /**
    It converts the input spikes into spike events:
    - Spike Events are composed by vertical slices of the input spikes matrix
    - Each vertical slice is a vector of 0/1 representing the spikes received by each input neuron at a given time instant

    The method also checks the consistency of the input spikes matrix:
    - The number of input neurons must be consistent with the number of rows in the input spikes matrix
    - The number of columns in the input spikes matrix must be consistent for all the rows
    - The value of the spike must be either 0 or 1

    @param spikes (&Vec<Vec<u8>>)
    The input of the SNN is a matrix of 0/1, where each row represents the array of spikes received by each input neuron.

    @return Vec<SpikeEvent>
   */
  fn derive_input_spike_events(&self, spikes: &Vec<Vec<u8>>) -> Vec<SpikeEvent> {
    
    let mut spike_events: Vec<SpikeEvent> = Vec::new();

    // check the number of input neurons is consistent with the number of rows in the input spikes matrix
    if self.get_input_layer_num_neurons() != spikes.len() {
      panic!("The number of input neurons is not consistent with the number of rows in the input spikes matrix.");
    }

    // check the number of columns in the input spikes matrix is consistent for all the rows
    let num_time_steps = spikes[0].len();
    for i in 1..spikes.len() {
      if spikes[i].len() != num_time_steps {
        panic!("The number of columns in the input spikes matrix is not consistent for all the rows.");
      }
    }

    // derive the spike events
    for t in 0..num_time_steps {

      let mut spikes_t: Vec<u8> = Vec::new();
      
      // generate the vertical slice
      for n in 0..spikes.len() {
        
        let spike = spikes[n][t];
        // check the value of the spike is consistent
        if spike != 0 && spike != 1 {
          panic!("The value of the spike is neither 0 nor 1.");
        }
        spikes_t.push(spike);
      }

      spike_events.push(SpikeEvent::new(t as u64, spikes_t));
    }

    spike_events
  }

  /**
    This method converts the final output spike events (resulting from the final inference)
    into a matrix of 0/1, where each row represents the array of spikes produced by each output neuron.

    It can be cconsidered as the inverse of the generate_input_spike_events method.

    @param output_spike_events (&Vec<SpikeEvent>)
    @return Vec<Vec<u8>>
   */
  fn derive_output_spikes(&self, output_spike_events: &Vec<SpikeEvent>) -> Vec<Vec<u8>> {

    let num_rows = self.get_output_layer_num_neurons();
    let num_cols = output_spike_events.len();
    let mut output_spikes: Vec<Vec<u8>> = vec![vec![0; num_cols]; num_rows];

    // derive the output spikes
    for spike_event in output_spike_events {
      for (n, spike) in spike_event.get_spikes().iter().enumerate() {
        output_spikes[n][spike_event.get_t() as usize] = *spike;
      }
    }
  
    output_spikes
  }
  

}