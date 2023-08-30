# Spiking neural networks and resilience

## Description
The goal of this project is to create the interface of a Spiking neural network 
and analyze its resilience considering single bit faults.

The training phase of the network has not been carried out, 
assuming that the trained hyperparameters are already available.

It has been realized for the course "Programmazione di sistema" of Politecnico di Torino, a.y. 2022-2023.

## Group members
- Beatrice Occhiena
- Matilde Zampolini

## Project structure
The repository is structured as follows:
- `src/` contains the source code of the library
  - `network/` contains the SNN generic implementation
    - `event` contains the spike event definition
    - `neuron` contains the generic neuron trait definition and the Lif neuron implementation
  - `resilience/` contains the SNN resilience implementation
- `tests/` contains the tests for the library without any fault

## Main structures
- A generic configuration has been realized using a `Neuron` trait (i.e. an interface) that outlines the common methods and behaviours expected from any neuron model.
```rust
pub trait Neuron {
    fn process_input(&mut self, time: u64, weighted_sum: f64, fault: Option<InjectedFault>) -> u8;
    fn initialize(&mut self);
}
```

- Distinct specific neuron models like LIF, IF, or AdEx can implement this trait, providing their own implementation for the required methods.
In this project the `Lif` neuron has been implemented:
```rust
pub struct Lif {
    reset_potential: f64, // reset potential
    resting_potential: f64, // resting potential
    threshold: f64, // threshold potential
    membrane_potential: f64, // membrane potential
    tau: f64, // time constant
    ts: u64 // last time instant where a spike has been received
}
```

- The neuron processes inputs in response to individual `SpikeEvent` rather than at fixed time intervals
```rust
pub struct SpikeEvent {
    t: u64, // time instant
    spikes: Vec<u8> // input vector (0/1) in a layer at time instant t
}
```

- The struct `Layer` represents a layer of neurons
```rust
pub struct Layer<N> where N: Neuron + Clone + Send + 'static {
    neurons: Vec<N>,                // neurons in a layer
    extra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the same layer
    prev_output: Vec<u8>,           // output vector (0/1) in a layer at time instant t-1: used to decrease the v_mem according to the intra_weights
}
```

## Main methods
```rust
fn process_input(&mut self, time: u64, weighted_sum: f64) -> u8;
```
This method signature can be applied to many neuron models, particularly those that compute the weighted sum of inputs and use a simple threshold-based firing mechanism.

## Usage example

First draft of this project