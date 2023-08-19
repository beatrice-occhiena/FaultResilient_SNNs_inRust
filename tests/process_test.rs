use std::vec;
use group02::network::config::SNNBuilder;
use group02::network::neuron::lif::Lif;

#[test]
fn test_add_layers() {
    let snn = SNNBuilder::<Lif>::new(0)
        .add_layer(Vec::new(), Vec::new(), Vec::new())
        .add_layer(Vec::new(), Vec::new(), Vec::new())
        .add_layer(Vec::new(), Vec::new(), Vec::new())
        .build();
    assert_eq!(snn.get_num_layers(),3);
}

#[test]
fn test_parameters_layers_with_neurons() {
    let snn_params = SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
                vec![0.1, 0.2, 0.3],
                vec![0.4, 0.5, 0.6]], vec![
                    vec![0.0, -0.2],
                    vec![-0.8, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            ], vec![
                vec![0.1, 0.2]], vec![
                    vec![-0.1]
        ]).get_parameters();

    assert_eq!(snn_params.get_extra_weights().get(0).is_some(), true);
    assert_eq!(snn_params.get_extra_weights().get(1).is_some(), true);
    assert_eq!(snn_params.get_extra_weights().get(2).is_none(), true);
    assert_eq!(snn_params.get_intra_weights().get(0).is_some(), true);
    assert_eq!(snn_params.get_intra_weights().get(0).is_some(), true);
    assert_eq!(snn_params.get_intra_weights().get(2).is_none(), true);
    assert_eq!(snn_params.get_neurons().get(1).is_some(), true);
    assert_eq!(snn_params.get_neurons().get(2).is_some(), false);

    assert_eq!(snn_params.get_extra_weights().get(0).unwrap(), &[[0.1, 0.2, 0.3],[0.4, 0.5, 0.6]]);
    assert_eq!(snn_params.get_intra_weights().get(1).unwrap(), &[[-0.1]]);
    assert_eq!(snn_params.get_neurons().get(0).unwrap().len(), 2);
    assert_eq!(snn_params.get_neurons().get(1).unwrap().len(), 1);
}

#[test]
fn test_snn_layers() {
    let snn = SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6]], vec![
            vec![0.0, -0.2],
            vec![-0.9, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
        ], vec![
            vec![0.2, 0.3]], vec![
            vec![0.0]
        ]).build();
    assert_eq!(snn.get_num_layers(),2);
    assert_eq!(snn.get_layers().get(0).is_some(), true);
    assert_eq!(snn.get_layers().get(1).is_some(), true);
    assert_eq!(snn.get_layers().get(2).is_some(), false);
    assert_eq!(snn.get_layers().get(0).unwrap().lock().unwrap().get_neurons().len(), 2);
    assert_eq!(snn.get_layers().get(1).unwrap().lock().unwrap().get_neurons().len(), 1);
    assert_eq!(snn.get_layers().get(0).unwrap().lock().unwrap().get_extra_weights().len(), 2);
}
/*
#[test]
#[should_panic]
fn test_negative_extra_weights() { // this test should panic
    SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![-0.1, 0.2, 0.3], // negative extra-weight
            vec![0.4, 0.5, 0.6]], vec![
            vec![0.0, -0.2],
            vec![-0.9, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
        ], vec![
            vec![0.2, 0.3]], vec![
            vec![0.0]
        ]);
}
*/
#[test]
#[should_panic]
fn test_positive_intra_weights() { // this test should panic
    SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6]], vec![
            vec![0.0, 0.2], // positive intra-weight
            vec![-0.9, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
        ], vec![
            vec![0.2, 0.3]], vec![
            vec![0.0]
        ]);
}

#[test]
#[should_panic]
fn test_len_extra_weights() { // this test should panic
    SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2, 0.3, 0.2], // wrong len
            vec![0.4, 0.5, 0.6, 0.8]], vec![
            vec![0.0, -0.2],
            vec![-0.9, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
        ], vec![
            vec![0.2, 0.3]], vec![
            vec![0.0]
        ]);
}

#[test]
#[should_panic]
fn test_len_extra_weights2() { // this test should panic
    SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6]], vec![
            vec![0.0, -0.2],
            vec![-0.9, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
        ], vec![
            vec![0.2, 0.3, 0.4]], vec![ // wrong len
            vec![0.0]
        ]);
}

#[test]
#[should_panic]
fn test_len_intra_weights() { // this test should panic
    SNNBuilder::new(3)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6]], vec![
            vec![0.0, -0.2, -0.9], // wrong len
            vec![-0.9, 0.0, -0.2]
        ])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
        ], vec![
            vec![0.2, 0.3]], vec![
            vec![0.0]
        ]);
}

#[test]
#[should_panic]
fn test_snn_with_no_layers() {
    SNNBuilder::<Lif>::new(0).build();
}

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

    let output_spikes = snn.process_input(&vec![vec![1,0,1],vec![0,0,1]], None);
    //let output_expected: Vec<Vec<u8>> = vec![vec![0,0,0],vec![1,0,1],vec![1,0,1]];
    let output_expected: Vec<Vec<u8>> = vec![vec![0,0,0],vec![0,0,1],vec![1,0,1]];
    assert_eq!(output_spikes, output_expected);
}

#[test]
fn test_process_snn_with_more_layers() {
    let snn = SNNBuilder::new(2)
        .add_layer(vec![
            Lif::new(0.2, 0.1, 0.5, 0.7),
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.1, 0.2],
            vec![0.3, 0.4]], vec![
            vec![0.0, -0.4],
            vec![-0.1, 0.0]
        ])
        .add_layer(vec![
            Lif::new(0.15, 0.1, 0.2, 0.1),
            Lif::new(0.05, 0.2, 0.3, 0.3),
            Lif::new(0.1, 0.15, 0.4, 0.8),
            Lif::new(0.01, 0.35, 0.05, 1.0)], vec![
            vec![0.7, 0.2],
            vec![0.3, 0.8],
            vec![0.5, 0.6],
            vec![0.3, 0.2]], vec![
            vec![0.0, -0.2, -0.4, -0.9],
            vec![-0.1, 0.0, -0.3, -0.2],
            vec![-0.6, -0.2, 0.0, -0.9],
            vec![-0.5, -0.3, -0.8, 0.0]])
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0)], vec![
            vec![0.3, 0.3, 0.2, 0.7]], vec![
            vec![0.0]])
        .build();

    let output_spikes = snn.process_input(&vec![vec![1,0,1,0],vec![0,0,1,1]], None);
    //let output_expected: Vec<Vec<u8>> = vec![vec![1,0,1,1]];
    let output_expected: Vec<Vec<u8>> = vec![vec![0,0,1,1]];

    assert_eq!(output_spikes, output_expected);
}

#[test]
fn test_process_snn_with_only_one_input() {
    let snn = SNNBuilder::new(2)
        .add_layer(vec![
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0),
            Lif::new(0.1, 0.05, 0.3, 1.0)],
        vec![
            vec![0.1, 0.2],
            vec![0.3, 0.4],
            vec![0.5, 0.25]
        ],vec![
        vec![0.0, -0.1, -0.15],
        vec![-0.05, 0.0, -0.1],
        vec![-0.15, -0.1, 0.0]
    ]).build();

    let output_spikes = snn.process_input(&vec![vec![0],vec![1]], None);
    let output_expected: [[u8; 1]; 3] = [[0],[1],[0]];

    assert_eq!(output_spikes, output_expected);
}

#[test]
#[should_panic]
fn test_snn_wrong_input_spikes() {
    let snn = SNNBuilder::new(2)
        .add_layer(vec![
            Lif::new(0.84, 0.05, 0.3, 1.0),
            Lif::new(0.12, 0.87, 0.3, 0.89)], vec![
            vec![0.12, 0.5],
            vec![0.53, 0.43]], vec![
            vec![0.0, -0.3],
            vec![-0.4, 0.0]
        ]).build();

    let _output_spikes = snn.process_input(&vec![vec![0,4],vec![0,1]], None);
}