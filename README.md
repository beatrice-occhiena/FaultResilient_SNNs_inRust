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
- `simulation/` contains the Python files that trains the network weights, the input spike trains 
based on the MNIST dataset and the labels used to compute the accuracy

## Network architecture
### Neuron
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

### Layer
- `Layer` is a struct that represents a layer of neurons
```rust
pub struct Layer<N> where N: Neuron + Clone + Send + 'static {
    neurons: Vec<N>,                // neurons in a layer
    extra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<f64>>,   // weights of the connections between each neuron and the neurons in the same layer
    prev_output: Vec<u8>,           // output vector (0/1) in a layer at time instant t-1: used to decrease the v_mem according to the intra_weights
}
```
- `SpikeEvent` is a struct that represents the firing event of a layer at a given time, 
i.e. the sum of the output of each neuron contained in it.
```rust
pub struct SpikeEvent {
    t: u64, // time instant
    spikes: Vec<u8> // input vector (0/1) in a layer at time instant t
}
```
#### Processing steps
1. **Initialization**: Initial reset of all the neurons in the layer.
2. **Listening to Spike Events**: The method continuously listens for incoming spike events from 
the previous layer through the **`layer_input_rc`** receiver channel.
3. **Processing Spike Events**: For each received **`input_spike_event`**, the method:
  - Saves the corresponding time instant
  - Calculates the weighted input sum for each neuron and triggers the computation of the membrane potential
  - Adds the resulting neuron output to the output vector
4. **Propagation of Output Spikes**: If at least one neuron fired, it proceeds to send the computed 
Spike Event to the next layer through the **`layer_output_tx`** sender channel.
5. **Saving Output Spikes**: The computed **`output_spikes`** are saved in the **`prev_output_spikes`** 
vector, which will be used in the next iteration to compute the **`intra_weighted_sum`** for the next round of processing.
6. **Finalization**: The method continues to listen for more incoming spike events from the previous 
layer until the channel is closed or the previous layer terminates. Once all input spike events are 
processed, the method completes its execution.


### SNN
`SNN` is the struct that represents a Spiking Neural Network composed by a vector of layers
```rust
pub struct SNN < N: Neuron + Clone + Send + 'static > {
    layers:  Vec<Arc<Mutex<Layer<N>>>>,
}
```
#### Processing phase
- Pre-processing
```rust
fn derive_input_spike_events(&self, input_spikes: &Vec<Vec<u8>>) -> Vec<SpikeEvent>
```
The method checks the consistency of the input spikes matrix:
1. The number of input neurons must be consistent with the number of rows in the input spikes matrix 
2. The number of columns in the input spikes matrix must be consistent for all the rows 
3. The value of the spike must be either 0 or 1
The input spike matrix is converted into a vector of Spike Events

- Parallel-processing
```rust
fn process_input_spike_events(&self, input_spike_events: Vec<SpikeEvent>) -> Vec<SpikeEvent>
```
Steps:
1. **create the first channel**: create the initial input channel to feed the network.
2. `create_and_spawn_threads`: For each layer, clone its Arc pointer, create an additional channel 
to the next layer, create a new thread that starts the layer processing.
3. `send_input_spike_events`: Send the input spike events to the first layer through the input channel.
4. `wait_for_threads`: Waits for all the spawned threads to finish.
5. `receive_output_spike_events`: Receive the output spike events from the last layer and collects them in a vector.

- Post-processing
```rust
fn derive_output_spikes(&self, output_spike_events: &Vec<SpikeEvent>, spike_len: usize) -> Vec<Vec<u8>>
```
The vector of Spike Events resulting from the final inference is converted in a simple matrix of spikes.


### Builder
The builder module is used to create and configure an SNN: it collects parameters for each layer, 
it performs consistency checks and finally builds the SNN.

`BuilderParameters` is a struct that contains all the parameters describing the network
```rust
pub struct BuilderParameters<N: Neuron> {
    input_length: usize,                // dimension of the network input layer
    neurons: Vec<Vec<N>>,               // neurons in each layer
    extra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the same layer
    num_layers: usize,                  // number of layers
}
```

## Configuration file
The configuration file `config.toml` contains parameters and settings for building a neural network 
with specific characteristics. It includes details about the network architecture, neuron configurations, 
weight files, input spike trains, and output file locations.
- `NetworkSetup` is a struct that holds parsed network configuration parameters based on `config.toml` file
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

TO_DO


## Tool interface
`Iced` framework for Rust has been used to realize an interface for the user to select the properties 
of the faults to inject and study the resilience of the Spiking Neural Network.

The trait `Application` is the main entrypoint of Iced. Once implemented, the GUI application can be run by simply calling run.

The trait requires 4 methods: 
- `new` initializes the Application with the flags provided to run as part of the Settings
- `title` returns the current title of the Application
- `update` handles a message and updates the state of the Application.
- `view` returns the widgets to display in the Application.

The steps of the Application are described by the following structure:
```rust
pub struct Steps {
  steps: Vec<Step>, 
  current: usize,
}
```
`Step` is defined as an enum. Each element contains a struct composed by the variables that are required at that step.

Each step is associated with a method that returns a Column (i.e., a container that distributes its contents vertically). 

The following objects of the crate has been used: `text`, `text_input`, `radio`, `checkbox`.

TO_DO

## Resilience analysis
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


## Usage example