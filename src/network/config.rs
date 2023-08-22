extern crate toml;
use std::fs::File;
use std::io::Read;

// NetworkSetup and Parsing from Config File
// -----------------------------------------
// This file defines the `NetworkSetup` struct and functions for parsing network configuration from a TOML file.
// - `NetworkSetup` struct holds parsed network configuration parameters.
// - `network_setup_from_file` reads and parses the TOML config file and returns a `NetworkSetup` object.

#[derive(Debug)]
pub struct NetworkSetup {
    pub input_length: usize,
    pub hidden_layers_lengths: Vec<usize>,
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
    pub input_spike_train: String
}

impl NetworkSetup {
    fn new(input_length: usize, hidden_layers_lengths: Vec<usize>, output_length: usize, extra_weights: Vec<String>, intra_weights: Vec<String>, resting_potential: f64, reset_potential: f64, threshold: f64, beta: f64, tau: f64, spike_length: usize, batch_size: usize, input_spike_train: String) -> Self{
        NetworkSetup {input_length, hidden_layers_lengths, output_length, extra_weights, intra_weights, resting_potential, reset_potential, threshold, beta, tau, spike_length, batch_size, input_spike_train}
    }
}

pub fn network_setup_from_file() -> Result<NetworkSetup, &'static str> {

    // Read the configuration file
    let mut config_file = File::open("src/config.toml").expect("Failed to open config file");
    let mut config_toml = String::new();
    config_file.read_to_string(&mut config_toml).expect("Failed to read config file");

    // Parse the TOML configuration
    //let c = toml::from_str(&config_toml);
    let config: toml::Value;// = toml::from_str(&config_toml).expect("Failed to parse TOML config");
    match toml::from_str(&config_toml) {
        Ok(c) => config = c,
        Err(_e) => return Err("Error")
    }

    // Access parameters

    // NETWORK DIMENSIONS
    let input_length = config["input_layer"]["input_length"].as_integer().unwrap() as usize;
    let hidden_layers_lengths = config["hidden_layers"]["neurons"]
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

    println!("Input Length: {}", input_length);
    println!("Hidden Layers: {:?}", hidden_layers_lengths);
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

     Ok(NetworkSetup::new(input_length.clone(), hidden_layers_lengths.clone(), output_length.clone(), extra_weights.clone(), intra_weights.clone(), resting_potential.clone(), reset_potential.clone(), threshold.clone(), beta.clone(), tau.clone(), spike_length.clone(), batch_size.clone(), input_spike_train.clone()))

    // Now you can use the extracted parameters to build your SNN and perform operations as needed.
}