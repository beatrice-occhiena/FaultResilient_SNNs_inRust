use std::sync::mpsc::{Sender, Receiver};
use crate::network::neuron::neuron::Neuron;
use crate::network::event::spike_event::SpikeEvent;
use crate::resilience::components::{ComponentType, ComponentCategory};
use crate::resilience::fault_models::InjectedFault;


#[derive(Debug)]
pub struct Layer<N> 
where N: Neuron + Clone + Send + 'static
{
    neurons: Vec<N>,                    // neurons in a layer
    pub extra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the previous layer
    pub intra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the same layer
    prev_output: Vec<u8>,               // output vector (0/1) in a layer at time instant t-1: used to decrease the v_mem according to the intra_weights
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
      prev_output: vec![0; num_n] 
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

  pub fn get_prev_output(&self) -> &Vec<u8> {
    &self.prev_output
  }

  pub fn get_tot_num_extra_weights(&self) -> usize {
    let num_rows = self.extra_weights.len();
    let num_cols = self.extra_weights[0].len();
    num_rows * num_cols
  }

  pub fn get_tot_num_intra_weights(&self) -> usize {
    let num_rows = self.intra_weights.len();
    let num_cols = self.intra_weights[0].len();
    num_rows * num_cols
  }

  /**
      It returns the number of components of the given type in the layer.
   **/
  pub fn get_num_components_from_type(&self, component_type: &ComponentType) -> usize {
    match component_type {
      // select one weight from the corresponding weights matrix
      ComponentType::Extra => self.get_tot_num_extra_weights(),
      ComponentType::Intra => self.get_tot_num_intra_weights(),
      // else select one neuron from the neuron vector
      _ => self.get_num_neurons(),
    }
  }

  // Setters
  fn initialize (&mut self) {
    self.prev_output.clear();
    self.prev_output = vec![0; self.neurons.len()];
    
    for neuron in self.neurons.iter_mut() {
      neuron.initialize();
    }
  }

  /**
    It processes the input spikes coming from the previous layer
    according to the model of the neurons in the layer, and returns 
    the output spikes to the next layer.
    - @param input_rc: the channel to receive the input spike event from the previous layer
    - @param output_tx: the channel to send the output spike event to the next layer
   */
  pub fn process_input(&mut self, input_rc: Receiver<SpikeEvent>, output_tx: Sender<SpikeEvent>, fault: Option<InjectedFault>) {
    
    // reset the neurons in the layer to reuse the SNN
    // for future inferences without building a new one
    self.initialize();

    // listen to the input spikes from the previous layer
    // until an Err is received (the channel is closed)
    while let Ok(input) = input_rc.recv() {

      let timestamp = input.get_t();
      let input_spikes = input.get_spikes();
      let mut output_spikes = Vec::<u8>::with_capacity(self.neurons.len());

      // if all the spikes in the output vector are 0
      // then there is no need to send the output spikes to the next layer
      let mut all_zero = true;

      let extra_len = self.extra_weights[0].len();
      let intra_len = self.intra_weights[0].len();

      // for each neuron in the layer compute the membrane potential
      // and check if it spikes
      // -----------------------------------------------------------
      for (i, neuron) in self.neurons.iter_mut().enumerate() {

        // compute the sum of the weights of the connections between the neuron 
        // and the neurons in the previous layer 
        // ---> we consider the input spikes
        let mut extra_weights_sum = 0.0;
        for (j, weight) in self.extra_weights[i].iter().enumerate() {
          
          // If the fault targets the extra weight selected => apply the fault
          if fault.is_some()
            && fault.unwrap().component_type == ComponentType::Extra 
            && fault.unwrap().component_index == (i*extra_len + j)
          {
            extra_weights_sum += fault.unwrap().apply_fault_f64(*weight, timestamp) * input_spikes[j] as f64;
          }
          else {
            extra_weights_sum += weight * input_spikes[j] as f64;
          }
        }

        // compute the sum of the weights of the connections between the neuron
        // and the neurons in the same layer
        // ---> we consider the output spikes of the previous time instant
        // - !!! ATTENTION to not consider the reflexive links from a neuron to itself !!!
        let mut intra_weights_sum = 0.0;
        for (j, weight) in self.intra_weights[i].iter().enumerate() {
          if i != j {
            if fault.is_some()
              && fault.unwrap().component_type == ComponentType::Intra 
              && fault.unwrap().component_index == (i*intra_len + j)
            {
              intra_weights_sum += fault.unwrap().apply_fault_f64(*weight, timestamp) * self.prev_output[j] as f64;
            }
            else {
              intra_weights_sum += weight * self.prev_output[j] as f64;
            }

          }
        }

        let weights_sum = extra_weights_sum + intra_weights_sum;

        // compute the membrane potential and check if it spikes
        // and update the output spikes vector
        let spike;
        if fault.is_some()
          && fault.unwrap().component_category != ComponentCategory::Connection
          && fault.unwrap().component_index == i
        { // the fault still has to be injected in this neuron
          spike = neuron.process_input(timestamp, weights_sum, fault);
        } 
        else 
        { // there is no fault to be injected in this neuron
          spike = neuron.process_input(timestamp, weights_sum, None);
        }
        output_spikes.push(spike);

        // update the flag to send the output spikes to the next layer
        if all_zero && spike == 1u8 {
          all_zero = false;
        }
      }

      // update the output vector of the previous time instant
      // for the next iteration
      self.prev_output = output_spikes.clone();

      // if at least one spike in the input vector is 1
      // then the output spikes are sent to the next layer
      if !all_zero{

        let output = SpikeEvent::new(timestamp, output_spikes);
        output_tx.send(output).unwrap();
      } 
    }
  }

}