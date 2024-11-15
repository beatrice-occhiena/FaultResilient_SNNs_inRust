use crate::network::neuron::neuron::Neuron;
use crate::resilience::components::{ComponentType, ComponentCategory};
use crate::resilience::fault_models::{InjectedFault, ApplyFault};
// Implements the Neuron trait with the leaky integrate-and-fire (LIF) model.

#[derive(Debug)]
pub struct Lif {
    reset_potential: f64, // reset potential
    resting_potential: f64, // resting potential
    threshold: f64, // threshold potential
    membrane_potential: f64, // membrane potential
    tau: f64, // time constant
    ts: u64, // last time instant where a spike has been received
    dt: f64 // quantization parameter that defines the amplitude of the interval between two consecutive instants
}

impl Lif {
    pub fn new(reset_potential: f64, resting_potential: f64, threshold: f64, tau: f64, dt: f64) -> Self {
        Lif {
            reset_potential,
            resting_potential,
            threshold,
            membrane_potential: resting_potential, // at the beginning the membrane potential is equal to the resting potential
            tau,
            ts: 0, // starting from time instant 0
            dt
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
    fn process_input(&mut self, time: u64, mut weighted_sum: f64, fault: Option<InjectedFault>) -> u8 {

        // Get the parameters of the neuron checking during runtime if there is a fault to inject
        // => In this way we are not soiling the original values of the built network with the fault,
        //    since it's sufficient to inject the fault only when the neuron is processed
        let (reset_potential,resting_potential, threshold, membrane_potential, tau, dt, ts)
            = self.read_memory_areas(fault, time);

        // Possible fault in the adder/multiplier
        // #to_do: CHECK CORRECTNESS !!!
        if fault.is_some() && (fault.unwrap().component_type == ComponentType::Adder || fault.unwrap().component_type == ComponentType::Multiplier) {
            weighted_sum = fault.unwrap().apply_fault(weighted_sum, time);
        }

        // Compute the membrane potential at the time instant t
        let mut output_spike: u8;
        let delta_t = ((time - ts) as f64) * dt; // time interval between two input spikes
        let exponential = (-delta_t/tau) as f64;
        let mp = resting_potential + (membrane_potential - resting_potential) * exponential.exp() + weighted_sum;

        // update the variables to be stored each step in the memory areas
        if fault.is_some() && fault.unwrap().component_type == ComponentType::MembranePotential{
            self.membrane_potential = fault.unwrap().apply_fault(mp, time);
        } else {
            self.membrane_potential = mp;
        }
        self.ts = time;

        // Check if the neuron spikes
        if self.membrane_potential > threshold {
            self.membrane_potential = reset_potential;
            output_spike = 1; // spike only if v_mem > v_th
        }
        else {
            output_spike = 0;
        }

        // Possible fault in the threshold comparator
        // - stuck-at-0: the neuron never spikes
        // - stuck-at-1: the neuron always spikes
        // - bit-flip: the neuron spikes when v_mem < v_th
        if fault.is_some() && fault.unwrap().component_type == ComponentType::ThresholdComparator{
            output_spike = fault.unwrap().apply_fault(output_spike, time);
        }

        output_spike
    }
    
    // Reset the membrane potential to the resting potential and the time instant to 0
    fn initialize(&mut self) {
        self.membrane_potential = self.resting_potential;
        self.ts = 0;
    }

    // Get the value of the parameter to fault
    fn get_parameter_to_fault(&mut self, component_type: ComponentType) -> &mut f64{
        match component_type {
            ComponentType::ResetPotential       => &mut self.reset_potential,
            ComponentType::RestingPotential     => &mut self.resting_potential,
            ComponentType::Threshold            => &mut self.threshold,
            ComponentType::MembranePotential    => &mut self.membrane_potential,
            ComponentType::Tau                  => &mut self.tau,
            ComponentType::DT                   => &mut self.dt,
            //ComponentType::Ts                   => &mut self.ts,
            _                                   => panic!("Error: the component type is not valid for the LIF neuron"),
        }   
    }

}

impl Lif {
    fn read_memory_areas(&mut self, fault: Option<InjectedFault>, time: u64) -> (f64, f64, f64, f64, f64, f64, u64) {
        // Get the parameters of the neuron
        // => In this way we are not soiling the original values of the built network with the fault,
        //    since it's sufficient to inject the fault only when the neuron is processed
        let mut reset_potential = self.reset_potential;
        let mut resting_potential = self.resting_potential;
        let mut threshold = self.threshold;
        let mut membrane_potential = self.membrane_potential;
        let mut tau = self.tau;
        let mut ts = self.ts;
        let mut dt = self.dt;
    
        if let Some(injected_fault) = fault {
            if injected_fault.component_category == ComponentCategory::MemoryArea {
                match injected_fault.component_type {
                    ComponentType::ResetPotential       => reset_potential = injected_fault.apply_fault(reset_potential, time),
                    ComponentType::RestingPotential     => resting_potential = injected_fault.apply_fault(resting_potential, time),
                    ComponentType::Threshold            => threshold = injected_fault.apply_fault(threshold, time),
                    ComponentType::MembranePotential    => membrane_potential = injected_fault.apply_fault(membrane_potential, time),
                    ComponentType::Tau                  => tau = injected_fault.apply_fault(tau, time),
                    ComponentType::Ts                   => ts = injected_fault.apply_fault(ts, time),
                    ComponentType::DT                   => dt = injected_fault.apply_fault(dt, time),
                    _                                   => {}
                }
            }
        }
        (reset_potential, resting_potential, threshold, membrane_potential, tau, dt, ts)
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
            dt: self.dt
        }
    }
}