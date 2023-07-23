use group02::network::event::spike_event::SpikeEvent;
use group02::network::neuron::lif::Lif;
use group02::network::neuron::neuron::Neuron;

fn main() {
    let v : Vec<u8> = vec![0,1];
    let i = SpikeEvent::new(0,v);
    println!("{:?}", i);
    let mut l = Lif::new(1.1,2.2,3.4,4.2,0.7,1u64);
    println!("{:?}", l);
    l.set_v_reset(0.5);
    println!("{:?}", l);
    let spike = l.process_input(1,10.6);
    println!("spike = {}", spike);
}