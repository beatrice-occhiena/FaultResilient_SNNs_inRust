#[derive(Debug)]
pub struct SpikeEvent {
    t: u64, // time instant
    spikes: Vec<u8> // input vector (0/1) in a layer at time instant t
}

impl SpikeEvent {
    pub fn new(t: u64, spikes: Vec<u8>) -> Self {
        SpikeEvent {
            t,spikes
        }
    }

    // Getters
    pub fn get_t(&self) -> u64 { self.t }
    pub fn get_spikes(&self) -> &Vec<u8> { &self.spikes }
}