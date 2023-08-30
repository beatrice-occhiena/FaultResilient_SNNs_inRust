# Spiking Neural Networks and resilience

## Description
The goal of this project is to create the interface of a Spiking Neural Network 
and analyze its resilience considering single bit faults.

The training phase of the network has not been carried out, 
assuming that the trained hyperparameters are already available.

The project has been realized for the course "Programmazione di sistema" of Politecnico di Torino, a.y. 2022-2023.

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
- `simulation/` contains the Python files that trains the network weights, the input spike trains based on the MNIST dataset and the labels used to compute the accuracy

## Main structures
- `Neuron` is the trait used to realize a generic configuration that outlines the common methods and behaviours expected from any neuron model
```rust
pub trait Neuron {
    fn process_input(&mut self, time: u64, weighted_sum: f64, fault: Option<InjectedFault>) -> u8;
    fn initialize(&mut self);
}
```

- `Lif` is the struct that describes the parameters of a Leaky Integrate-and-Fire neuron
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

- `SpikeEvent` is a struct that represents the inputs processes by the neurons
```rust
pub struct SpikeEvent {
    t: u64, // time instant
    spikes: Vec<u8> // input vector (0/1) in a layer at time instant t
}
```

- `Layer` is a struct that represents a layer of neurons
```rust
pub struct Layer<N> where N: Neuron + Clone + Send + 'static {
    neurons: Vec<N>,                // neurons in a layer
    extra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the same layer
    prev_output: Vec<u8>,           // output vector (0/1) in a layer at time instant t-1: used to decrease the v_mem according to the intra_weights
}
```

- `SNN` is the struct that represents a Spiking Neural Network composed by a vector of `Layer`s
```rust
pub struct SNN < N: Neuron + Clone + Send + 'static > {
    layers:  Vec<Arc<Mutex<Layer<N>>>>,
}
```

- `BuilderParameters` is a struct that contains all the parameters describing the network
```rust
pub struct BuilderParameters<N: Neuron> { //struct that contains all the parameters describing the network
    input_length: usize,                // dimension of the network input layer
    neurons: Vec<Vec<N>>,               // neurons in each layer
    extra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the same layer
    num_layers: usize,                  // number of layers
}
```

- `NetworkSetup` is a struct that holds parsed network configuration parameters based on the `config.toml` file
```rust
pub struct NetworkSetup {
    pub input_layer: usize,
    pub hidden_layers: Vec<usize>,
    pub output_length: usize,
    pub extra_weights: Vec<String>,
    pub intra_weights: Vec<String>,
    pub resting_potential: f64,
    pub reset_potential: f64,
    pub threshold: f64,
    pub beta: f64,
    pub tau: f64,
    pub spike_length: usize,
    pub batch_size: usize,
    pub input_spike_train: String,
    pub target_file: String
}
```

- `InjectedFault` is a struct representing a fault occurrence with its properties
```rust
pub struct InjectedFault {
    // FAULT PROPERTIES
    pub fault_type: FaultType,
    pub time_step: Option<u64>,               // Time step at which the fault must be injected (for transient bit-flip faults only)
    // FAULT LOCATION
    pub layer_index: usize,                     // Layer index of the component in which the fault must be injected
    pub component_category: ComponentCategory,  // Category of component in which the fault must be injected
    pub component_type: ComponentType,          // Type of component in which the fault must be injected
    pub component_index: usize,                 // Index of the component in which the fault must be injected
    pub bit_index: Option<usize>,               // Bit index of the component in which the fault must be injected (not for threshold comparators)
}
```

- `UserSelection` is a struct to hold the fault injection parameters defined by the user
```rust
pub struct UserSelection {
    pub components: Vec<ComponentType>,
    pub fault_type: FaultType,
    pub num_faults: u64,
    pub input_sequence: Vec<Vec<Vec<u8>>>,
}
```

## Main methods


## Usage example

First draft of this project