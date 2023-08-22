use std::sync::{Arc, Mutex};
use crate::network::layer::Layer;
use crate::network::neuron::neuron::Neuron;
use crate::network::snn::SNN;
use crate::network::config::NetworkSetup;

// SNNBuilder and Building a Spiking Neural Network
// ------------------------------------------------
// This module allows to create a complex object, that is the processable Spiking Neural Network (SNN).
// - `BuilderParameters` is a struct that contains all the parameters describing the network.
// - `SNNBuilder` is used to create and configure an SNN:
//    - It collects parameters for each layer
//    - It performs consistency checks
//    - And finally builds the SNN.

/**
     (the network) providing an interface
    to specify all the parameters that describe it. The user can specify:
    - Layers
    - Neurons (with the relative parameters) of each layer
    - Extra-weights and intra-weights
 **/

#[derive(Clone)]
pub struct BuilderParameters<N: Neuron> { //struct that contains all the parameters describing the network
    input_length: usize,                // dimension of the network input layer
    neurons: Vec<Vec<N>>,               // neurons in each layer
    extra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the previous layer
    intra_weights: Vec<Vec<Vec<f64>>>,  // weights of the connections between each neuron and the neurons in the same layer
    num_layers: usize,                  // number of layers
}

impl<N: Neuron + Clone> BuilderParameters<N> {
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
    parameters: BuilderParameters<N>
}

impl<N: Neuron + Clone + Send> SNNBuilder<N> {
    pub fn new(input_length: usize) -> Self {
        SNNBuilder {
            parameters: BuilderParameters {
                input_length,
                neurons: Vec::new(),
                extra_weights: Vec::new(),
                intra_weights: Vec::new(),
                num_layers: 0
            }
        }
    }

    pub fn get_parameters(&self) -> BuilderParameters<N> {
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