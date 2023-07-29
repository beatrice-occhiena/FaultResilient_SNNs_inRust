use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use crate::network::layer::Layer;
use crate::network::neuron::neuron::Neuron;
use crate::network::event::spike_event::SpikeEvent;


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
    self.layers[0].lock().unwrap().get_extra_weights().first().unwrap().len()
  }

  fn get_output_layer_num_neurons(&self) -> usize {
    self.layers[self.layers.len()-1].lock().unwrap().get_num_neurons()
  }

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
    
    In the pre-processing phase, the input spikes are converted into spike events 
    while checking their consistency.

    @return Vec<Vec<u8>>
    The output of the SNN is a matrix of 0/1, where each row represents the array of spikes produced by each output neuron.
   */
  pub fn process_input(&self, spikes: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    // PRE-PROCESSING: convert the input spikes into spike events
    let input_spike_events = self.derive_input_spike_events(spikes);

    // PARALLEL PROCESSING: process the input spike events
    let output_spike_events = self.process_input_spike_events(input_spike_events);
    //let output_spike_events = self.verbose_process_input_spike_events(input_spike_events);

    // POST-PROCESSING: convert the output spike events into output spikes
    let output_spikes = self.derive_output_spikes(&output_spike_events, spikes.first().unwrap().len());

    output_spikes   
  }

  // PRE-PROCESSING PHASE
  // --------------------
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
  fn derive_input_spike_events(&self, input_spikes: &Vec<Vec<u8>>) -> Vec<SpikeEvent> {
    
    let mut spike_events: Vec<SpikeEvent> = Vec::new();

    // check the number of input neurons is consistent with the number of rows in the input spikes matrix
    if self.get_input_layer_num_neurons() != input_spikes.len() {
      panic!("The number of input neurons is not consistent with the number of rows in the input spikes matrix.");
    }

    // check the number of columns in the input spikes matrix is consistent for all the rows
    let num_time_steps = input_spikes[0].len();
    for i in 1..input_spikes.len() {
      if input_spikes[i].len() != num_time_steps {
        panic!("The number of columns in the input spikes matrix is not consistent for all the rows.");
      }
    }

    // derive the spike events
    for t in 0..num_time_steps {

      let mut spikes_t: Vec<u8> = Vec::new();
      
      // generate the vertical slice
      for n in 0..input_spikes.len() {
        
        let spike = input_spikes[n][t];
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

  // PARALLEL PROCESSING PHASE
  // -------------------------
  /**
    It processes input spike events and produces the resulting output spike events
      => final inference

    @param input_spike_events (Vec<SpikeEvent>)
    The input of the SNN is a vector of SpikeEvent, where each SpikeEvent represents the array of spikes received by each input neuron at a given time instant.

    @return Vec<SpikeEvent>
    The output of the SNN is a vector of SpikeEvent, where each SpikeEvent represents the array of spikes produced by each output neuron at a given time instant.
   */
  #[allow(dead_code)]
  fn verbose_process_input_spike_events(&self, input_spike_events: Vec<SpikeEvent>) -> Vec<SpikeEvent> {

    let mut thread_handles = Vec::<JoinHandle<()>>::new();

    // create the first channel for the input
    let (input_tx, mut layer_rc) = channel::<SpikeEvent>();    

    // for each layer create a new thread and process the input
    for layer in self.layers.iter() {

      // clone the Arc pointer to the layer 
      let layer = layer.clone();

      // create another channel for its communication with the next layer
      let (layer_tx, next_layer_rc) = channel::<SpikeEvent>();
      
      // create a new thread
      let handle = thread::spawn(move || {
        let mut layer = layer.lock().unwrap();
        layer.process_input(layer_rc,layer_tx);
      });

      // push the handle in the vector
      thread_handles.push(handle);

      // update the channel for the next layer
      layer_rc = next_layer_rc;
    }

    // the last channel is the output channel
    let output_rc = layer_rc;

    // send the input spike events to the first layer
    // (only if there is at least one spike with value 1)
    for spike_event in input_spike_events {
      if spike_event.get_spikes().iter().any(|&spike| spike == 1) {

        let time_istant = spike_event.get_t();

        input_tx.send(spike_event)
          .expect(&format!("Failed to send the input spike event to the first layer at t = {}.", time_istant));
      }
    }

    // close the input channel to terminate the communication
    drop(input_tx);

    // wait for the threads to finish
    for handle in thread_handles {
      handle.join().unwrap();
    }

    // receive the output spike events from the last layer
    let mut output_spike_events: Vec<SpikeEvent> = Vec::new();
    while let Ok(spike_event) = output_rc.recv() {
      output_spike_events.push(spike_event);
    }

    output_spike_events
  }

  fn process_input_spike_events(&self, input_spike_events: Vec<SpikeEvent>) -> Vec<SpikeEvent> {
    
    // Step 1: create the first channel for the input
    let (input_tx, layer_rc) = channel::<SpikeEvent>();

    // Step 2: Create and spawn threads
    let (thread_handles, output_rc) = self.create_and_spawn_threads(layer_rc);

    // Step 3: Send input spike events to the first layer
    SNN::<N>::send_input_spike_events(input_spike_events, input_tx);

    // Step 4: Wait for the threads to finish
    SNN::<N>::wait_for_threads(thread_handles);

    // Step 5: Receive output spike events from the last layer
    SNN::<N>::receive_output_spike_events(output_rc)
  }

  fn create_and_spawn_threads(&self, layer_rc: Receiver<SpikeEvent>) -> (Vec<JoinHandle<()>>, Receiver<SpikeEvent>) {
    
    let mut curr_layer_rc = layer_rc;
    
    let mut thread_handles = Vec::<JoinHandle<()>>::new();
    
    // for each layer create a new thread and activate the processing of the input
    for layer in self.layers.iter() {
        // clone the Arc pointer to the layer 
        let layer = layer.clone();

        // create another channel for its communication with the next layer
        let (curr_layer_tx, next_layer_rc) = channel::<SpikeEvent>();

        // create a new thread
        let handle = thread::spawn(move || {
            let mut layer = layer.lock().unwrap();
            layer.process_input(curr_layer_rc,curr_layer_tx);
        });

        // push the handle in the vector
        thread_handles.push(handle);

        // update the channel for the next layer
        curr_layer_rc = next_layer_rc;
    }

    // the last channel is the output channel
    let output_rc = curr_layer_rc;

    (thread_handles, output_rc)
}

fn send_input_spike_events(input_spike_events: Vec<SpikeEvent>, input_tx: Sender<SpikeEvent>) {
    // send the input spike events to the first layer
    // (only if there is at least one spike with value 1)
    for spike_event in input_spike_events {
        if spike_event.get_spikes().iter().any(|&spike| spike == 1) {
            let time_istant = spike_event.get_t();

            input_tx
                .send(spike_event)
                .expect(&format!("Failed to send the input spike event to the first layer at t = {}.", time_istant));
        }
    }
    drop(input_tx);
}

fn wait_for_threads(thread_handles: Vec<JoinHandle<()>>) {
    // wait for the threads to finish
    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn receive_output_spike_events(layer_rc: Receiver<SpikeEvent>) -> Vec<SpikeEvent> {
    // receive the output spike events from the last layer
    let mut output_spike_events: Vec<SpikeEvent> = Vec::new();
    while let Ok(spike_event) = layer_rc.recv() {
        output_spike_events.push(spike_event);
    }

    output_spike_events
}


  // POST-PROCESSING PHASE
  // ---------------------
  /**
    This method converts the final output spike events (resulting from the final inference)
    into a matrix of 0/1, where each row represents the array of spikes produced by each output neuron.

    It can be considered as the inverse of the generate_input_spike_events method.

    @param output_spike_events (&Vec<SpikeEvent>)
    @return Vec<Vec<u8>>
   */
  fn derive_output_spikes(&self, output_spike_events: &Vec<SpikeEvent>, spike_len: usize) -> Vec<Vec<u8>> {

    let num_rows = self.get_output_layer_num_neurons();
    let num_cols = spike_len;
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