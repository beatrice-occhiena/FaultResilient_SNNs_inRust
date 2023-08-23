use crate::network::snn::SNN;

extern crate toml;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use crate::network::builder::SNNBuilder;
use crate::network::neuron::lif::Lif;

// NetworkSetup and Parsing from Config File
// -----------------------------------------
// This file defines the `NetworkSetup` struct and functions for parsing network configuration from a TOML file.
// This module defines the `NetworkSetup` struct and functions for parsing network configuration from a TOML file.
// - `NetworkSetup` struct holds parsed network configuration parameters.
// - `network_setup_from_file` reads and parses the TOML config file and returns a `NetworkSetup` object.
// It also converts the parsed parameters to a fully configured `SNN` object usinf the `SNNBuilder` module.

#[derive(Debug)]
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

impl NetworkSetup {
    fn new(input_layer: usize, hidden_layers: Vec<usize>, output_length: usize, extra_weights: Vec<String>, intra_weights: Vec<String>, resting_potential: f64, reset_potential: f64, threshold: f64, beta: f64, tau: f64, spike_length: usize, batch_size: usize, input_spike_train: String, target_file: String) -> Self{
        NetworkSetup {input_layer, hidden_layers, output_length, extra_weights, intra_weights, resting_potential, reset_potential, threshold, beta, tau, spike_length, batch_size, input_spike_train, target_file}
    }
}

pub fn network_setup_from_file() -> Result<NetworkSetup, &'static str> {

    // Read the configuration file
    let mut config_file = File::open("src/config.toml").expect("Failed to open config file");
    let mut config_toml = String::new();
    config_file.read_to_string(&mut config_toml).expect("Failed to read config file");

    // Parse the TOML configuration
    let config: toml::Value;// = toml::from_str(&config_toml).expect("Failed to parse TOML config");
    match toml::from_str(&config_toml) {
        Ok(c) => config = c,
        Err(_e) => return Err("Error")
    }

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

    // WEIGHT FILES -> check if length = hidden_layers_length
    let weight_files = config["weight_files"].as_table().unwrap();
    let extra_weights = weight_files["extra_weights"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
    // optional intra_weights
    let intra_weights;
    if weight_files.contains_key("intra_weights") {
        intra_weights = config["weight_files"]["intra_weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<String>>();
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
        beta = (-1.0 / tau).exp() as f64;
    }

    // INPUT SPIKES PARAMETERS
    let spike_length = config["input_spike_train"]["spike_length"].as_integer().unwrap() as usize;
    let batch_size = config["input_spike_train"]["batch_size"].as_integer().unwrap() as usize;
    let input_spike_train = config["input_spike_train"]["filename"].to_string();

    // TARGET FILE FOR ACCURACY
    let target_file = config["accuracy"]["target_file"].to_string();

    // Use the parameters to build your network and perform operations
    let input_layer = input_length;
    let mut hidden_layers = hidden_layers_length;
    hidden_layers.push(output_length);

     Ok(NetworkSetup::new(input_layer.clone(), hidden_layers.clone(), output_length.clone(), extra_weights.clone(), intra_weights.clone(), resting_potential.clone(), reset_potential.clone(), threshold.clone(), beta.clone(), tau.clone(), spike_length.clone(), batch_size.clone(), input_spike_train.clone(), target_file.clone()))
    // Now you can use the extracted parameters to build your SNN and perform operations as needed.
}

pub fn build_network_from_setup(n: NetworkSetup) -> (SNN<Lif>, Vec<Vec<Vec<u8>>>, Vec<u8>) {
    // Building neurons
    let mut vec_neurons = Vec::new();
    for l in n.hidden_layers.iter() {
        vec_neurons.push(get_neurons(*l, n.reset_potential, n.resting_potential, n.threshold, n.tau));
    }

    // Getting extra_weights from files
    let mut vec_extra_weights = Vec::new();
    let w = get_extra_weights(rem_first_and_last(n.extra_weights.get(0).unwrap().as_str()), n.input_layer, *n.hidden_layers.get(0).unwrap());
    vec_extra_weights.push(w);
    for (i,extra_file) in n.extra_weights.iter().enumerate() {
        if i > 0 && i < n.extra_weights.len() - 1{
            vec_extra_weights.push(get_extra_weights(rem_first_and_last(extra_file.as_str()), *n.hidden_layers.get(i).unwrap(), *n.hidden_layers.get(i+1).unwrap()));
        }
    }
    vec_extra_weights.push(get_extra_weights(rem_first_and_last(n.extra_weights.get(n.extra_weights.len() - 1).unwrap().as_str()), *n.hidden_layers.get(n.hidden_layers.len() - 2).unwrap(), n.output_length));

    // Building intra_weights -> DA RIFARE
    let mut vec_intra_weights = Vec::new();
    for l in n.hidden_layers.iter() {
        vec_intra_weights.push(get_intra_weights(*l));
    }

    //Building the SNN
    let mut snn_builder = SNNBuilder::new(n.input_layer);
    for (w, n) in vec_extra_weights.iter().zip(vec_intra_weights.iter()).zip(vec_neurons.iter()) {
        snn_builder = snn_builder.add_layer(n.clone().to_vec(), w.0.clone(), w.1.clone());
    }
    let snn = snn_builder.build();

    // Getting input spike trains from file
    let input_spike_train = get_input_spike_train(rem_first_and_last(n.input_spike_train.as_str()), n.input_layer, n.spike_length, n.batch_size);

    // Getting targets from file
    let targets = get_targets(rem_first_and_last(n.target_file.as_str()), n.batch_size);

    (snn, input_spike_train, targets)
}

fn get_neurons(num_neurons: usize, reset_potential: f64, resting_potential: f64, threshold: f64, tau: f64) -> Vec<Lif> {
    // Building the vector of Lif with the parameters received as arguments
    let mut neurons = Vec::new();
    for _ in 0..num_neurons {
        neurons.push(Lif::new(reset_potential, resting_potential, threshold, tau));
    }
    neurons
}

fn get_extra_weights(filename: &str, input_length: usize, num_neurons: usize) -> Vec<Vec<f64>> {
    // Opening the file
    let f = File::open(filename).expect("Error: The weight file doesn't exist");
    // Initialize the matrix of weights to all zeros
    let mut extra_weights = vec![vec![0f64; input_length]; num_neurons];
    // Reading the file by lines
    let reader = BufReader::new(f);
    for (i,line) in reader.lines().enumerate() {
        // Each line is a String -> I have to split it and convert to f64
        let mut j = 0;
        for w in line.unwrap().split(" ") {
            if w != "" {
                extra_weights[i][j] = w.parse::<f64>().expect("Cannot convert to f64");
                j+=1;
            }
        }
    }
    extra_weights
}

fn get_intra_weights(num_neurons: usize) -> Vec<Vec<f64>> {
    // The intra weights are not stored in a file but are all set to the value 0.0
    let w = 0.0;
    let mut intra_weights = vec![vec![0f64; num_neurons]; num_neurons];
    for i in 0..num_neurons {
        for j in 0..num_neurons {
            if i != j {
                intra_weights[i][j] = w;
            }
        }
    }
    intra_weights
}

fn get_input_spike_train(filename: &str, input_length: usize, spike_length: usize, batch_size: usize) -> Vec<Vec<Vec<u8>>> {
    // Opening the file
    let f = File::open(filename).expect("Error: The file spikeTrains.txt doesn't exist");
    // Initialize the matrix of input spikes to all zeros
    let mut spike_trains = vec![vec![vec![0u8; spike_length]; input_length]; batch_size];
    // Reading the file by lines
    let reader = BufReader::new(f);
    let mut k = 0;
    for (i,line) in reader.lines().enumerate() {
        // Each line is a String -> I have to split it and convert to f64
        if i==0 || line.as_ref().unwrap().eq("# New slice") {
            k+=1;
        }
        else {
            let lu8 = line.unwrap().chars().filter(|c| *c != ' ').map(|c|  {
                c.to_digit(10).unwrap() as u8
            }).collect::<Vec<u8>>();
            for (j, w) in lu8.into_iter().enumerate() {
                spike_trains[k-1][j][i-k-(k-1)*spike_length] = w;
            }
        }
    }
    spike_trains
}

pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn get_targets(filename: &str, batch_size: usize) -> Vec<u8> {
    // Opening the file
    let f = File::open(filename).expect("Error: The targets file doesn't exist");
    // Initialize the matrix of weights to all zeros
    let mut targets = vec![0u8; batch_size];
    // Reading the file by lines
    let reader = BufReader::new(f);
    for (i,line) in reader.lines().enumerate() {
        targets[i] = line.unwrap().parse::<u8>().expect("Cannot convert to u8");
    }
    targets
}