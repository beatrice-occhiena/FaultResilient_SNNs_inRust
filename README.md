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
    fn get_parameter_to_fault(&mut self, component_type: ComponentType) -> &mut f64;
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
    dt: f64 // quantization parameter that defines the amplitude of the interval between two consecutive instants
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
The method checks the consistency of the input spikes matrix that is converted into a vector of Spike Events.

- Parallel-processing
```rust
fn process_input_spike_events(&self, input_spike_events: Vec<SpikeEvent>) -> Vec<SpikeEvent>
```
The method follows these steps:
1. create the first channel: create the initial input channel to feed the network.
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
`SNNBuilder` is a struct that allows the user to specify all the parameters needed to construct the SNN object layer by layer,
providing methods such as: `new`, `add_layer`, `build`.
```rust
pub struct SNNBuilder<N: Neuron> {
    parameters: BuilderParameters<N>
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
    pub dt: f64, 
    pub spike_length: usize,
    pub batch_size: usize,
    pub input_spike_train: String,
    pub target_file: String
}
```


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
Each step is associated with a method that defines the container that is displayed. 

The following objects of the crate has been used: `text`, `text_input`, `radio`, `checkbox`, `image`.


## Resilience analysis
For the resilience analysis only single-bit faults have been considered (stuck-at-0, stuck-at-1, transient-bit-flip).

`InjectedFault` is a struct representing a fault occurrence with its properties
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
`ComponentType` is an enum that contains the components of the network in which a fault can be introduced
```rust
pub enum ComponentType {
    // Connections between neurons
    Extra,
    Intra,
    // LIF Memory areas
    ResetPotential,
    RestingPotential,
    Threshold,
    MembranePotential,
    Tau,
    Ts,
    DT, 
    // Internal processing blocks
    Adder,
    Multiplier,
    ThresholdComparator,
}
```
The following trait defines the generic function that allows to apply a fault in a bit of the selected variable. The trait is implemented for f64, u64 and u8.
```rust
pub trait ApplyFault<T> {
    fn apply_fault(&self, var: T, timestamp: u64) -> T;
}
```

A graphic interface has been created for the user to provide the properties of the fault to insert in the network.
`UserSelection` is a struct to hold the fault injection parameters defined by the user
```rust
pub struct UserSelection {
    pub components: Vec<ComponentType>,
    pub fault_type: FaultType,
    pub num_faults: u64,
    pub input_sequence: Vec<Vec<Vec<u8>>>,
}
```
Given the user selection, the following function select a random bit index from the components selected and runs the simulation of the SNN with the injected fault.
It returns a vector of tuples containing: the accuracy of the SNN with the injected faults and all the information about the injected fault.
```rust
pub fn run_simulation(&self, user_selection: UserSelection, targets: Vec<u8>, no_faults_accuracy: f64) -> Vec<(f64,InjectedFault)>
```

The accuracy is computed with the following function:
```rust
pub fn compute_accuracy(vec_max: Vec<u8>, targets: &Vec<u8>) -> f64
```
It sums the spikes over time and compare the neuron with the highest number of spikes with the target.

## Usage example
This is an example of test to test the network without faults:
```rust
fn test_process_snn_with_more_layers() {
    let snn = SNNBuilder::new(2)
        .add_layer(vec![
            Lif::new(0.2, 0.1, 0.5, 0.7, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0, 1.0)], vec![
            vec![0.1, 0.2],
            vec![0.3, 0.4]], vec![
            vec![0.0, -0.4],
            vec![-0.1, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.15, 0.1, 0.2, 0.1, 1.0),
            Lif::new(0.05, 0.2, 0.3, 0.3, 1.0),
            Lif::new(0.1, 0.15, 0.4, 0.8, 1.0),
            Lif::new(0.01, 0.35, 0.05, 1.0, 1.0)], vec![
            vec![0.7, 0.2],
            vec![0.3, 0.8],
            vec![0.5, 0.6],
            vec![0.3, 0.2]], vec![
            vec![0.0, -0.2, -0.4, -0.9],
            vec![-0.1, 0.0, -0.3, -0.2],
            vec![-0.6, -0.2, 0.0, -0.9],
            vec![-0.5, -0.3, -0.8, 0.0]])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0, 1.0)], vec![
            vec![0.3, 0.3, 0.2, 0.7]], vec![
            vec![0.0]])
        .build();

    let output_spikes = snn.process_input(&vec![vec![1,0,1,0],vec![0,0,1,1]], None);
    let output_expected: Vec<Vec<u8>> = vec![vec![1,0,1,1]];

    assert_eq!(output_spikes, output_expected);
}
```
This is an example of test to see how the accuracy changes injecting a significant fault in the threshold parameter of the LIF neuron:
```rust
fn test_positive_threshold_fault_injection() {

  let n = network_setup_from_file();
  let (snn, input_spike_train, targets) = build_network_from_setup(n.unwrap());

  // MANUAL FAULT INJECTION
  //***************************************************************************
  let fault_type: FaultType = FaultType::StuckAt1;
  let time_step: Option<u64> = None;
  let layer_index: usize = 1;
  let component_category: ComponentCategory = ComponentCategory::MemoryArea;
  let component_type: ComponentType = ComponentType::Threshold;
  let component_index: usize = 2;
  let bit_index: Option<usize> = Some(62);
  //***************************************************************************
  let fault = InjectedFault::new(fault_type, time_step, layer_index, component_type, component_category, component_index, bit_index);

  // PROCESSING WITH FAULT INJECTION
  let mut vec_max = Vec::new();
  for input_spikes in input_spike_train.iter() {
      let output_spikes = snn.process_input(&input_spikes, Some(fault));
      let max = compute_max_output_spike(output_spikes);
      vec_max.push(max);
  }
  let acc = compute_accuracy(vec_max, &targets);
  println!("Accuracy = {}%", acc);

  // PRINT RESULTS
  println!(""); // empty line
  println!("Injected fault info:");
  println!("{:?}", fault);
  println!("Resulting accuracy = {}%", acc);
  println!(""); // empty line

}
```