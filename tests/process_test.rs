use group02::network::config::SNNBuilder;
use group02::network::neuron::lif::Lif;

#[test]
fn test_process_snn_one_layer() {
    let snn = SNNBuilder::new(2)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2],
            vec![0.3, 0.4],
            vec![0.5, 0.6]], vec![
            vec![0.0, -0.1, -0.15],
            vec![-0.05, 0.0, -0.1],
            vec![-0.15, -0.1, 0.0]
        ])
        .build();

    let output_spikes = snn.process_input(&vec![vec![1,0,1],vec![0,0,1]]);
    let output_expected: Vec<Vec<u8>> = vec![vec![0,0,0],vec![1,0,1],vec![1,0,1]];
    assert_eq!(output_spikes, output_expected);
}