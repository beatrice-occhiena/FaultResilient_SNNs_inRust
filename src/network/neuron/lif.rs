use crate::network::neuron::neuron::Neuron;
use crate::resilience::components::ComponentType;
use crate::resilience::fault_models::InjectedFault;
// Implements the Neuron trait with the leaky integrate-and-fire (LIF) model.

#[derive(Debug)]
pub struct Lif {
    reset_potential: f64, // reset potential
    resting_potential: f64, // resting potential
    threshold: f64, // threshold potential
    membrane_potential: f64, // membrane potential
    tau: f64, // time constant
    ts: u64 // last time instant where a spike has been received
}

impl Lif {
    pub fn new(reset_potential: f64, resting_potential: f64, threshold: f64, tau: f64) -> Self {
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
    fn process_input(&mut self, time: u64, weighted_sum: f64, fault: Option<InjectedFault>) -> u8 {

        // Get the parameters of the neuron
        // => In this way we are not soiling the original values of the built network with the fault,
        //    since it's sufficient to inject the fault only when the neuron is processed
        let mut reset_potential = self.reset_potential;
        let mut resting_potential = self.resting_potential;
        let mut threshold = self.threshold;
        let mut membrane_potential = self.membrane_potential;
        let mut tau = self.tau;
        let mut ts = self.ts;

        // Check if the neuron is faulty in one of its components
        if fault.is_some() {
            if fault.unwrap().component_type == ComponentType::ResetPotential {
                reset_potential = fault.unwrap().apply_fault_f64(reset_potential, time);
            }
            else if fault.unwrap().component_type == ComponentType::RestingPotential {
                resting_potential = fault.unwrap().apply_fault_f64(resting_potential, time);
            }
            else if fault.unwrap().component_type == ComponentType::Threshold {
                threshold = fault.unwrap().apply_fault_f64(threshold, time);
            }
            else if fault.unwrap().component_type == ComponentType::MembranePotential {
                membrane_potential = fault.unwrap().apply_fault_f64(membrane_potential, time);
            }
            else if fault.unwrap().component_type == ComponentType::Tau {
                tau = fault.unwrap().apply_fault_f64(tau, time);
            }
            else if fault.unwrap().component_type == ComponentType::Ts {
                ts = fault.unwrap().apply_fault_u64(ts, time);
            }
        }

        let dt = (time - ts) as f64; // time interval between two input spikes
        let exponential = (-dt/tau) as f64;
        self.membrane_potential = resting_potential + (membrane_potential - resting_potential) * exponential.exp() + weighted_sum;
        self.ts = time;
        if self.membrane_potential > threshold {
            self.membrane_potential = reset_potential;
            1 // spike only if v_mem > v_th
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

impl Clone for Lif {
    fn clone(&self) -> Self {
        Lif {
            reset_potential: self.reset_potential,
            resting_potential: self.resting_potential,
            threshold: self.threshold,
            membrane_potential: self.membrane_potential,
            tau: self.tau,
            ts: self.ts,
        }
    }
}