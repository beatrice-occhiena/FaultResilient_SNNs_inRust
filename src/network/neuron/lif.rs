use crate::network::neuron::neuron::Neuron;
// Implements the Neuron trait with the leaky integrate-and-fire (LIF) model.

#[derive(Debug)]
pub struct Lif{
    reset_potential: f64, // reset potential
    resting_potential: f64, // resting potential
    threshold: f64, // threshold potential
    membrane_potential: f64, // membrane potential
    tau: f64, // time constant
    ts: u64 // last time instant where a spike has been received
}

impl Lif {
    pub fn new(reset_potential: f64, resting_potential: f64, threshold: f64, membrane_potential: f64, tau: f64, ts: u64) -> Self {
        Lif {
            reset_potential,
            resting_potential,
            threshold,
            membrane_potential: resting_potential, // at the beginning the membrane potential is equal to the resting potential
            tau,
            ts: 0 // starting from time instant 0
        }
    }

     // Getters for the Lif parameters
    pub fn get_v_reset(&self) -> f64 { self.reset_potential }
    pub fn get_v_rest(&self) -> f64 { self.resting_potential }
    pub fn get_v_th(&self) -> f64 { self.threshold }
    pub fn get_v_mem(&self) -> f64 { self.membrane_potential }
    pub fn get_tau(&self) -> f64 { self.tau }
    pub fn get_ts(&self) -> u64 { self.ts }
    
    // Setters for potential parameters of Lif
    pub fn set_v_reset(&mut self, new_v_reset: f64) -> () { self.reset_potential = new_v_reset }
    pub fn set_v_rest(&mut self, new_v_rest: f64) -> () { self.resting_potential = new_v_rest }
    pub fn set_v_th(&mut self, new_v_th: f64) -> () { self.threshold = new_v_th }
}

impl Neuron for Lif {
    
    /**
        Computes the membrane potential of the neuron at the time instant t
        and returns 1 if the neuron spikes, 0 otherwise.
        - @param time (u64)
        - @param weights_sum (f64) #to_do: check if it is correct
        - @return u8 (0/1)
     */
    fn process_input(&mut self, time: u64, weighted_sum: f64) -> u8 {
        let dt = (time - self.ts) as f64; // time interval between two input spikes
        let exponential = (-dt/self.tau) as f64;
        self.membrane_potential = self.resting_potential + (self.membrane_potential - self.resting_potential) * exponential.exp() + weighted_sum;
        self.ts = time;
        if self.membrane_potential > self.threshold {
            self.membrane_potential = self.reset_potential;
            1 // spike only in if v_mem > v_th
        }
        else {
            0
        }
    }

    // Reset the membrane potential to the resting potential and the time instant to 0
    fn initialize(&mut self) {
        self.membrane_potential = self.resting_potential;
        self.ts = 0;
    }
}