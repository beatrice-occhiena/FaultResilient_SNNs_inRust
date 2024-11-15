#[warn(non_camel_case_types)]

/** Enum to represent the type of component for verification
 * 
The "**connections between neurons**" are represented by the 
  - **`extra_weights`** and
  - **`intra_weights`** 
fields in the Layer struct. extra_weights holds the weights of the connections between each neuron and the 
neurons in the previous layer, while intra_weights holds the weights of the connections between each neuron 
and the neurons in the same layer. These weights are used to compute the weighted sum of inputs during the 
neuron's processing.

The "**memory areas**" are represented by various fields in the **Lif** struct, such as 
  - **`reset_potential`**, 
  - **`resting_potential`**, 
  - **`threshold`**, 
  - **`membrane_potential`**, 
  - **`tau`**, and 
  - **`ts`**. 
These parameters are fundamental for the functioning of individual neurons, 
and they hold important numerical values that govern the neuron's dynamics.

To perform resilience analysis on "**internal processing blocks within the neuron**," we need to simulate faults 
in the individual components responsible for 
  - `**summation**`, 
  - `**multiplication**`, and 
  - `**threshold comparison**`. 
These faults can be of the stuck-at-0, stuck-at-1, or transient bit-flip type, as described in the project requirements. 
For instance, simulating a stuck-at-1 fault in the threshold comparator would mean the neuron always spikes, even when 
the threshold condition is not met.
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComponentCategory {
  Connection,
  MemoryArea,
  InternalProcessingBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl ComponentType{

  pub fn get_category(&self) -> ComponentCategory {
    match self {
      ComponentType::Extra |
      ComponentType::Intra => ComponentCategory::Connection,

      ComponentType::Adder |
      ComponentType::Multiplier |
      ComponentType::ThresholdComparator => ComponentCategory::InternalProcessingBlock,

      _ => ComponentCategory::MemoryArea,
    }
  }

  /**
   * Returns true if the component is a static component 
   * (i.e. a component whose value does not change during the simulation).
   */
  pub fn is_static_component(&self) -> bool{
    match self {
      ComponentType::Extra |
      ComponentType::Intra |
      ComponentType::ResetPotential |
      ComponentType::RestingPotential |
      ComponentType::Threshold |
      ComponentType::Tau |
      ComponentType::DT => true,
      _ => false,
    }
  }
}