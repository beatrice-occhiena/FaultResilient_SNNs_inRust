use std::sync::{Arc, Mutex};
use crate::network::layer::Layer;
use crate::network::neuron::neuron::Neuron;
use crate::network::snn::SNN;

extern crate toml;
use std::fs::File;
use std::io::Read;

pub fn network_setup_from_file(){

    // Read the configuration file
    let mut config_file = File::open("src/network/config.toml").expect("Failed to open config file");
    let mut config_toml = String::new();
    config_file.read_to_string(&mut config_toml).expect("Failed to read config file");

    // Parse the TOML configuration
    let config: toml::Value = toml::from_str(&config_toml).expect("Failed to parse TOML config");

    // Access parameters

    // NETWORK DIMENSIONS
    let input_length = config["input_layer"]["input_length"].as_integer().unwrap() as usize;
    let hidden_layers_length = config["hidden_layers"]["neurons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_integer().unwrap() as usize)
        .collect::<Vec<usize>>();
    let output_length = config["output_layer"]["neurons"].as_integer().unwrap() as usize;

    // WEIGHT FILES
    let weight_files = config["weight_files"].as_table().unwrap();
    let extra_weights = weight_files["extra_weights"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w.as_str().unwrap())
        .collect::<Vec<&str>>();
    // optional intra_weights
    let intra_weights;
    if weight_files.contains_key("intra_weights") {
        intra_weights = config["weight_files"]["intra_weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.as_str().unwrap())
            .collect::<Vec<&str>>();
    }else{
        intra_weights = Vec::new();
    }

    // NEURON PARAMETERS
    let lif_params = config["LIF_neuron_parameters"].as_table().unwrap();
    let resting_potential = lif_params["resting_potential"].as_float().unwrap() as f64;
    let reset_potential = lif_params["reset_potential"].as_float().unwrap() as f64;
    let threshold = lif_params["threshold"].as_float().unwrap() as f64;
    let beta;
    let tau;
    if lif_params.contains_key("beta"){
        beta = lif_params["beta"].as_float().unwrap() as f64;
        tau = (-1.0 / beta.ln()) as f64;
    }
    else {
        tau = lif_params["tau"].as_float().unwrap() as f64;
        beta = (-1.0 / tau.ln()) as f64;
    }

    // INPUT SPIKES PARAMETERS
    let spike_length = config["input_spike_train"]["spike_length"].as_integer().unwrap() as usize;
    let batch_size = config["input_spike_train"]["batch_size"].as_integer().unwrap() as usize;
    let input_spike_train = config["input_spike_train"]["filename"].as_str().unwrap();

    // Use the parameters to build your network and perform operations
    let input_layer = input_length;
    let mut hidden_layers = hidden_layers_length;
    hidden_layers.push(output_length);

    println!("Input Length: {}", input_layer);
    println!("Hidden Layers: {:?}", hidden_layers);
    println!("Output Length: {}", output_length);
    println!("Extra Weights: {:?}", extra_weights);
    println!("Intra Weights: {:?}", intra_weights);
    println!("Resting Potential: {}", resting_potential);
    println!("Reset Potential: {}", reset_potential);
    println!("Threshold: {}", threshold);
    println!("Beta: {}", beta);
    println!("Tau: {}", tau);
    println!("Spike Length: {}", spike_length);
    println!("Batch Size: {}", batch_size);
    println!("Input Spike Train: {}", input_spike_train);

    // Now you can use the extracted parameters to build your SNN and perform operations as needed.
}




/**
    This module allows to create a complex object (the network) providing an interface
    to specify all the parameters that describe it. The user can specify:
    - Layers
    - Neurons (with the relative parameters) of each layer
    - Extra-weights and intra-weights
 **/

#[derive(Clone)]
pub struct ParametersBuilder<N: Neuron> { //struct that contains all the parameters describing the network
    input_length: usize,                // dimension of the network input layer
    neurons: Vec<Vec<N>>,               // neurons in each layer
    extra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the same layer
    num_layers: usize,                  // number of layers
}

impl<N: Neuron + Clone> ParametersBuilder<N> {
    // Getters for builder parameters
    pub fn get_neurons(&self) -> Vec<Vec<N>> {
        self.neurons.clone()
    }
    pub fn get_extra_weights(&self) -> Vec<Vec<Vec<f64>>> {
        self.extra_weights.clone()
    }
    pub fn get_intra_weights(&self) -> Vec<Vec<Vec<f64>>> {
        self.intra_weights.clone()
    }
}

#[derive(Clone)]
pub struct SNNBuilder<N: Neuron> {
    parameters: ParametersBuilder<N>
}

impl<N: Neuron + Clone + Send> SNNBuilder<N> {
    pub fn new(input_length: usize) -> Self {
        SNNBuilder {
            parameters: ParametersBuilder {
                input_length,
                neurons: Vec::new(),
                extra_weights: Vec::new(),
                intra_weights: Vec::new(),
                num_layers: 0
            }
        }
    }

    pub fn get_parameters(&self) -> ParametersBuilder<N> {
        self.parameters.clone()
    }

    fn check_intra_weights(&self, intra_weights: &Vec<Vec<f64>>, neurons_len: usize) {
        if neurons_len != intra_weights.len() {
            panic!("Error: The number of neurons should be equal to the number of rows of the intra_weights matrix");
        }
        for row in intra_weights {
            if row.len() != neurons_len {
                panic!("Error: The number of neurons should be equal to the number of columns of the intra_weights matrix");
            }
            for w in row {
                if *w > 0.0 {
                    panic!("Error: Intra weights should be negative");
                }
            }
        }
    }

    fn check_extra_weights(&self, extra_weights: &Vec<Vec<f64>>, neurons_len: usize) {
        if neurons_len != extra_weights.len() {
            panic!("Error: The number of neurons should be equal to the number of rows of the extra_weights matrix");
        }
        for row in extra_weights {
            if self.parameters.num_layers == 0 {
                if self.parameters.input_length != row.len() {
                    panic!("Error: The number of columns of the extra_weights matrix should be equal to the dimension of the input layer if no layer exists")
                }
            }
            else {
                if row.len() != self.parameters.neurons[self.parameters.num_layers - 1].len() {
                    panic!("Error: The number of neurons in the previous layer should be equal to the number of columns of the extra_weights matrix");
                }
            }
            /*
            for w in row {
                if *w < 0.0 {
                    panic!("Error: Extra weights should be positive");
                }
            }
             */
        }
    }

    /**
        This method receives all the data for building a layer (neurons and intra and extra layer weights)
        and checks its consistency (at run-time)
     **/
    pub fn add_layer(self, neurons: Vec<N>, extra_weights: Vec<Vec<f64>>, intra_weights: Vec<Vec<f64>>) -> Self{
        // intra weights consistency check
        self.check_intra_weights(&intra_weights, neurons.len());
        // extra weights consistency check
        self.check_extra_weights(&extra_weights, neurons.len());

        // add parameters of the new layer
        let mut parameters = self.parameters;
        parameters.num_layers += 1;
        parameters.neurons.push(neurons);
        parameters.extra_weights.push(extra_weights);
        parameters.intra_weights.push(intra_weights);

        Self {
            parameters
        }
    }

    /**
        This method builds each layer of the SNN from the information collected
        by the SNNBuilder (neurons and weights)
    */
    pub fn build(self) -> SNN<N> {
        if self.parameters.num_layers == 0 {
            panic!("Error: The SNN must have at least one layer");
        }

        // Creation of each layer
        let mut layers = Vec::new();
        for (weights, neurons) in self.parameters.extra_weights.into_iter().zip(self.parameters.intra_weights).zip(self.parameters.neurons) {
            let layer = Layer::new(neurons, weights.0, weights.1);
            layers.push(Arc::new(Mutex::new(layer)));
        }
        SNN::new(layers)
    }

}