// possible GUI implementation with iced

use std::error::Error;
use iced::{alignment, Application, Color, executor, Theme, window};
use iced::theme::{self};
use iced::widget::{checkbox, column, container, horizontal_space, radio, row, text, text_input, Button, Column, TextInput, scrollable, image};
use iced::{Element, Length, Settings, Command};
use crate::network::config::{build_network_from_setup, compute_accuracy, compute_max_output_spike, network_setup_from_file};
use crate::network::neuron::lif::Lif;
use crate::network::snn::SNN;
use crate::resilience::components::ComponentType;
use crate::resilience::fault_models::{FaultType, InjectedFault};
use crate::resilience::simulation::UserSelection;
use plotters::prelude::*;
use plotters::style::Color as OtherColor;

pub fn launch() -> iced::Result {
    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
}

impl Tour {
    pub fn create_selection(&self) -> UserSelection {

        // User selection initialization
        let mut v = Vec::new();
        let mut fault = FaultType::StuckAt0;
        let mut num_faults= 0;
        let mut input_spike_train = Vec::new();

        // For each step of the GUI, we check what the user has selected
        for i in 1..self.steps.steps.len() {
            match self.steps.steps.get(i).unwrap() {
                Step::Components { intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator} => {
                    if *intra != false { v.push(ComponentType::Intra) }
                    if *extra != false { v.push(ComponentType::Extra) }
                    if *reset != false { v.push(ComponentType::ResetPotential) }
                    if *resting != false { v.push(ComponentType::RestingPotential) }
                    if *threshold != false { v.push(ComponentType::Threshold) }
                    if *vmem != false { v.push(ComponentType::MembranePotential) }
                    if *tau != false { v.push(ComponentType::Tau) }
                    if *ts != false { v.push(ComponentType::Ts) }
                    if *adder != false { v.push(ComponentType::Adder) }
                    if *multiplier != false { v.push(ComponentType::Multiplier) }
                    if *comparator != false { v.push(ComponentType::ThresholdComparator) }
                },
                Step::FaultType {selection} => {
                    fault = selection.unwrap();
                },
                Step::NumFaults {value} => {
                    num_faults = value.parse::<u64>().unwrap();
                },
                Step::Accuracy {input_spike_trains, ..} =>{
                    input_spike_train = (*input_spike_trains).clone();
                }
                _ => {}
            }
        }

        // Return the user selection
        UserSelection::new(v, fault, num_faults,input_spike_train)
    }

    pub fn get_arguments_for_simulation(&self) -> (UserSelection, Vec<u8>, SNN<Lif>, f64){
        
        let mut user_selection = UserSelection::new(vec![], FaultType::StuckAt0, 0, vec![]);
        let mut target = Vec::new();
        let mut snn_sim = SNN::new(Vec::new());
        let mut accuracy = 0.0;

        for i in 1..self.steps.steps.len() {
            match self.steps.steps.get(i).unwrap() {
                Step::Choices {c} => {
                    user_selection = (*c).clone();
                }
                Step::Accuracy {snn, targets, a, ..} => {
                    snn_sim = (*snn).clone();
                    target = (*targets).clone();
                    accuracy = (*a).clone();
                }
                _ => {}
            }
        }
        (user_selection, target, snn_sim, accuracy)
    }

    fn graphic(&self) -> Result<(), Box<dyn Error>> {
        let mut num_faults= 0;
        let mut a_f = Vec::new();
        // For each step of the GUI, we check what the user has selected
        for i in 1..self.steps.steps.len() {
            match self.steps.steps.get(i).unwrap() {
                Step::NumFaults {value} => {
                    num_faults = value.parse::<u64>().unwrap();
                },
                Step::Image {a_inj, ..} =>{
                    a_f = (*a_inj).clone();
                }
                _ => {}
            }
        }

        let mut x_values = Vec::new();
        for i in 0..num_faults {
            x_values.push(i as i32);
        }

        let root = BitMapBackend::new("plotters-data/graph.png", (1024, 768)).into_drawing_area();

        root.fill(&WHITE)?;

        let n = (num_faults-1) as i32;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Accuracy with faults", ("sans-serif", 50))
            .build_cartesian_2d(0..n, 0..100)?;

        chart
            .configure_mesh()
            .y_desc("Accuracy (%)")
            .x_desc("Fault number")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        let mut v = Vec::new();
        for i in 0..num_faults {
            v.push((x_values[i as usize] as i32, a_f[i as usize].0 as i32));
        }
        chart.draw_series(LineSeries::new(v.iter().map(|(i,j)| (*i, *j)), RED.filled())
            .point_size(5)).unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-data' dir exists under current dir");

        Ok(())
    }
}

impl Application for Tour {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Tour, Command<Message>) {
        (
            Tour {
                steps: Steps::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        format!("{} - Iced", self.steps.title())
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        
        match event {
            Message::BackPressed => {
                if self.steps.is_network() {
                    self.steps.update_step();
                }
                else {
                    self.steps.go_back();
                }
                Command::none()
            },
            Message::NextPressed => {
                self.steps.advance();

                // Build the network and test the accuracy without faults
                if self.steps.is_accuracy() {
                    let n = network_setup_from_file();
                    let (snn_net, input_spike_train, target) = build_network_from_setup(n.unwrap());
                    let s = &mut self.steps.steps[3];
                    match s {
                        Step::Accuracy { ref mut snn, ref mut input_spike_trains, ref mut targets , ref mut a}
                            => { *snn = snn_net;
                                *input_spike_trains = input_spike_train;
                                *targets = target;
                                let mut vec_max = Vec::new();
                                for input_spikes in (*input_spike_trains).iter() {
                                    let output_spikes = (*snn).process_input(&input_spikes, None);
                                    let max = compute_max_output_spike(output_spikes);
                                    vec_max.push(max);
                                 }
                                 let acc = compute_accuracy(vec_max, &(*targets));
                                *a = acc;
                        }
                        _ => {}
                    };
                }

                // #to_do: build the network and test the accuracy with faults
                if self.steps.is_choice() {
                    let user_selection = self.create_selection();
                    let s = &mut self.steps.steps[7]; // choices step
                    match s {
                        Step::Choices { ref mut c } => { *c = user_selection }
                        _ => {}
                    };
                }

                if self.steps.is_image() {
                    let (user_selection, targets, snn, accuracy) = self.get_arguments_for_simulation();
                    let v = snn.run_simulation(user_selection, targets, accuracy);
                    let s = &mut self.steps.steps[8];
                    match s {
                        Step::Image {ref mut a_inj} => {
                            *a_inj = v.clone();
                        }
                        _ => {}
                    }
                    let s1 = &mut self.steps.steps[9];
                    match s1 {
                        Step::Simulation {ref mut a_inj} => {
                            *a_inj = v.clone();
                        }
                        _ => {}
                    }
                    self.graphic().unwrap();
                }

                Command::none()
            },
            Message::UpdatePressed => {
                
                if self.steps.is_network(){
                    
                    if let Step::Network { 
                        input_length, hidden_layers_length, output_length, 
                        extra_files, intra_files, 
                        resting_potential, reset_potential, threshold, beta, tau, 
                        spike_length, batch_size, input_spike_train_file
                    } = self.steps.steps[1].clone() {

                        // read the network parameters from the configuration file
                        let mut n = network_setup_from_file().unwrap();

                        // check which parameters have been modified (not empty) in the GUI
                        if !input_length.is_empty() {
                            n.input_layer = input_length.parse::<usize>().unwrap();
                        }
                        if !hidden_layers_length.is_empty() {
                            let mut v = Vec::new();
                            let mut s = hidden_layers_length.clone();
                            s.retain(|c| c != '[' && c != ']');
                            let s = s.split(",");
                            for i in s {
                                v.push(i.parse::<usize>().unwrap());
                            }
                            n.hidden_layers = v;
                        }
                        if !output_length.is_empty() {
                            n.output_length = output_length.parse::<usize>().unwrap();
                        }
                        if !extra_files.is_empty() {
                            let mut v = Vec::new();
                            let mut s = extra_files.clone();
                            s.retain(|c| c != '[' && c != ']');
                            let s = s.split(",");
                            for i in s {
                                v.push(i.to_string());
                            }
                            n.extra_weights = v;
                        }
                        if !intra_files.is_empty() {
                            let mut v = Vec::new();
                            let mut s = intra_files.clone();
                            s.retain(|c| c != '[' && c != ']');
                            let s = s.split(",");
                            for i in s {
                                v.push(i.to_string());
                            }
                            n.intra_weights = v;
                        }
                        if !resting_potential.is_empty() {
                            n.resting_potential = resting_potential.parse::<f64>().unwrap();
                        }
                        if !reset_potential.is_empty() {
                            n.reset_potential = reset_potential.parse::<f64>().unwrap();
                        }
                        if !threshold.is_empty() {
                            n.threshold = threshold.parse::<f64>().unwrap();
                        }
                        if !beta.is_empty() {
                            n.beta = beta.parse::<f64>().unwrap();
                        }
                        if !tau.is_empty() {
                            n.tau = tau.parse::<f64>().unwrap();
                        }
                        if !spike_length.is_empty() {
                            n.spike_length = spike_length.parse::<usize>().unwrap();
                        }
                        if !batch_size.is_empty() {
                            n.batch_size = batch_size.parse::<usize>().unwrap();
                        }
                        if !input_spike_train_file.is_empty() {
                            n.target_file = input_spike_train_file.to_string();
                        }
                    
                        // update the configuration file
                        n.update_config_file();
                    }

                }
                // update the GUI window
                self.steps.update_step();
                Command::none()
            },
            Message::RestartPressed => {
                // Delete components
                let s = &mut self.steps.steps[4];
                match s {
                    Step::Components { ref mut intra, ref mut extra, ref mut reset,ref mut resting, ref mut threshold, ref mut vmem, ref mut tau, ref mut ts, ref mut adder, ref mut multiplier, ref mut comparator} => {
                        *intra = false;
                        *extra = false;
                        *reset = false;
                        *resting = false;
                        *threshold = false;
                        *vmem = false;
                        *tau = false;
                        *ts = false;
                        *adder = false;
                        *multiplier = false;
                        *comparator = false;
                    },
                    _ => {}
                };
                // Delete fault selection
                let s = &mut self.steps.steps[5];
                match s {
                    Step::FaultType {ref mut selection} => {
                        *selection = None
                    },
                    _ => {}
                }
                // Delete number of faults
                let s = &mut self.steps.steps[6];
                match s {
                    Step::NumFaults {ref mut value} => {
                        *value = String::new()
                    },
                    _ => {}
                }

                self.steps.restart();

                Command::none()
            },
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
                Command::none()
            },
            Message::ExitMessage => {
                window::close()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Tour { steps, .. } = self;

        let mut controls = row![];

        if steps.has_previous() && !steps.is_network() {
            controls = controls.push(button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        if steps.is_network() {
            controls = controls.push(button("Update")
                                         .on_press(Message::UpdatePressed)
                                         .style(theme::Button::Positive),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() && !steps.is_exit() {
            controls = controls.push(button("Next").on_press(Message::NextPressed));
        }

        if steps.is_exit() {
            controls = controls.push(button("Restart").on_press(Message::RestartPressed));
            controls = controls.push(button("Exit")
                .on_press(Message::ExitMessage)
                .style(theme::Button::Destructive),
            );
        }

        let content: Element<_> = column![
            steps.view().map(Message::StepMessage),
            controls,
        ].max_width(540).spacing(20).padding(20).into();

        let scrollable = scrollable(container(content)
                .width(Length::Fill)
                .center_x(),
        );

        container(scrollable).height(Length::Fill).center_y().into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    UpdatePressed,
    RestartPressed,
    StepMessage(StepMessage),
    ExitMessage
}

#[derive(Debug, Clone)]
pub struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Network{
                    input_length: String::new(), hidden_layers_length: String::new(), output_length: String::new(),
                    extra_files: String::new(), intra_files: String::new(),
                    resting_potential: String::new(), reset_potential: String::new(), threshold: String::new(), beta: String::new(), tau: String::new(),
                    spike_length: String::new(), batch_size: String::new(), input_spike_train_file: String::new(),
                },
                Step::Waiting,
                Step::Accuracy  {
                    snn: SNN::new(Vec::new()),
                    input_spike_trains: Vec::new(),
                    targets: Vec::new(),
                    a: 0.0
                },
                Step::Components {
                    intra: false, extra: false,
                    reset: false, resting: false, threshold: false, vmem: false, tau: false, ts: false,
                    adder: false, multiplier: false, comparator: false
                },
                Step::FaultType { selection: None },
                Step::NumFaults { value: String::new() },
                Step::Choices { c: UserSelection {
                    components: vec![],
                    fault_type: FaultType::StuckAt0,
                    num_faults: 0,
                    input_sequence: vec![],
                }},
                Step::Image {
                    a_inj: Vec::new()
                },
                Step::Simulation {
                    a_inj: Vec::new()
                },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn restart(&mut self) {
        self.current = 4; // Start from the beginning of the fault configuration
    }

    fn update_step(&mut self) {
        self.current = self.current;
    }

    fn is_choice(&self) -> bool {
        match self.steps[self.current] {
            Step::Choices { .. } => return true,
            _ => return false
        }
    }

    fn is_network(&self) -> bool {
        match self.steps[self.current] {
            Step::Network {..} => return true,
            _ => return false
        }
    }

    fn is_accuracy(&self) -> bool {
        match self.steps[self.current] {
            Step::Accuracy { .. } => return true,
            _ => return false
        }
    }

    fn is_image(&self) -> bool {
        match self.steps[self.current] {
            Step::Image { .. } => return true,
            _ => return false
        }
    }

    fn is_exit(&self) -> bool {
        self.current == self.steps.len() - 1
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

#[derive(Debug, Clone)]
enum Step {
    Welcome,
    Network{
        input_length: String, hidden_layers_length: String, output_length: String,
        extra_files: String, intra_files: String,
        resting_potential: String, reset_potential: String, threshold: String, beta: String, tau: String,
        spike_length: String, batch_size: String, input_spike_train_file: String,
    },
    Waiting,
    Accuracy {
        snn: SNN<Lif>,
        input_spike_trains: Vec<Vec<Vec<u8>>>,
        targets: Vec<u8>,
        a: f64
    },
    Components {
        intra: bool, extra: bool,
        reset: bool, resting: bool, threshold: bool, vmem: bool, tau: bool, ts: bool,
        adder: bool, multiplier: bool, comparator: bool,
    },
    FaultType { selection: Option<FaultType>, },
    NumFaults { value: String },
    Choices { c: UserSelection },
    Simulation {
        a_inj: Vec<(f64, InjectedFault)>
    },
    Image {
        a_inj: Vec<(f64, InjectedFault)>
    },
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    // Components selection
    IntraSelected(bool),
    ExtraSelected(bool),
    RstSelected(bool),
    RestSelected(bool),
    ThresholdSelected(bool),
    MemSelected(bool),
    TauSelected(bool),
    TsSelected(bool),
    AdderSelected(bool),
    MulSelected(bool),
    ComparatorSelected(bool),
    // Fault type selection
    FaultSelected(FaultType),
    // Number of faults selection
    InputChanged(String),
    // Network configuration parameters
    InputLengthChanged(String),
    HiddenLayersLengthChanged(String),
    OutputLengthChanged(String),
    ExtraFilesChanged(String),
    IntraFilesChanged(String),
    RestingPotentialChanged(String),
    ResetPotentialChanged(String),
    ThresholdChanged(String),
    BetaChanged(String),
    TauChanged(String),
    SpikeLengthChanged(String),
    BatchSizeChanged(String),
    InputSpikeTrainFileChanged(String),
}

impl<'a> Step {
    
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::IntraSelected(toggle) => {
                if let Step::Components { intra, .. } = self {
                    *intra = toggle;
                }
            }
            StepMessage::ExtraSelected(toggle) => {
                if let Step::Components { extra, .. } = self {
                    *extra = toggle;
                }
            }
            StepMessage::RstSelected(toggle) => {
                if let Step::Components { reset, .. } = self {
                    *reset = toggle;
                }
            }
            StepMessage::RestSelected(toggle) => {
                if let Step::Components { resting, .. } = self {
                    *resting = toggle;
                }
            }
            StepMessage::ThresholdSelected(toggle) => {
                if let Step::Components { threshold, .. } = self {
                    *threshold = toggle;
                }
            }
            StepMessage::MemSelected(toggle) => {
                if let Step::Components { vmem, .. } = self {
                    *vmem = toggle;
                }
            }
            StepMessage::TauSelected(toggle) => {
                if let Step::Components { tau, .. } = self {
                    *tau = toggle;
                }
            }
            StepMessage::TsSelected(toggle) => {
                if let Step::Components { ts, .. } = self {
                    *ts = toggle;
                }
            }
            StepMessage::AdderSelected(toggle) => {
                if let Step::Components { adder, .. } = self {
                    *adder = toggle;
                }
            }
            StepMessage::MulSelected(toggle) => {
                if let Step::Components { multiplier, .. } = self {
                    *multiplier = toggle;
                }
            }
            StepMessage::ComparatorSelected(toggle) => {
                if let Step::Components { comparator, .. } = self {
                    *comparator = toggle;
                }
            }
            StepMessage::FaultSelected(sel) => {
                if let Step::FaultType { selection } = self {
                    *selection = Some(sel);
                }
            }

            // Network configuration parameters
            StepMessage::InputChanged(new_value) => {
                if let Step::NumFaults { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::InputLengthChanged(new_value) => {
                if let Step::Network { input_length, .. } = self {
                    *input_length = new_value;
                }
            }
            StepMessage::HiddenLayersLengthChanged(new_value) => {
                if let Step::Network { hidden_layers_length, .. } = self {
                    *hidden_layers_length = new_value;
                }
            }
            StepMessage::OutputLengthChanged(new_value) => {
                if let Step::Network { output_length, .. } = self {
                    *output_length = new_value;
                }
            }
            StepMessage::ExtraFilesChanged(new_value) => {
                if let Step::Network { extra_files, .. } = self {
                    *extra_files = new_value;
                }
            }
            StepMessage::IntraFilesChanged(new_value) => {
                if let Step::Network { intra_files, .. } = self {
                    *intra_files = new_value;
                }
            }
            StepMessage::RestingPotentialChanged(new_value) => {
                if let Step::Network { resting_potential, .. } = self {
                    *resting_potential = new_value;
                }
            }
            StepMessage::ResetPotentialChanged(new_value) => {
                if let Step::Network { reset_potential, .. } = self {
                    *reset_potential = new_value;
                }
            }
            StepMessage::ThresholdChanged(new_value) => {
                if let Step::Network { threshold, .. } = self {
                    *threshold = new_value;
                }
            }
            StepMessage::BetaChanged(new_value) => {
                if let Step::Network { beta, .. } = self {
                    *beta = new_value;
                }
            }
            StepMessage::TauChanged(new_value) => {
                if let Step::Network { tau, .. } = self {
                    *tau = new_value;
                }
            }
            StepMessage::SpikeLengthChanged(new_value) => {
                if let Step::Network { spike_length, .. } = self {
                    *spike_length = new_value;
                }
            }
            StepMessage::BatchSizeChanged(new_value) => {
                if let Step::Network { batch_size, .. } = self {
                    *batch_size = new_value;
                }
            }
            StepMessage::InputSpikeTrainFileChanged(new_value) => {
                if let Step::Network { input_spike_train_file, .. } = self {
                    *input_spike_train_file = new_value;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::Network {..} => "Network",
            Step::Waiting => "Waiting",
            Step::Accuracy { .. } => "Accuracy",
            Step::Components { .. } => "Components",
            Step::FaultType {..} => "Fault",
            Step::Image {.. } => "Image",
            Step::NumFaults { .. } => "Number of faults",
            Step::Choices { .. } => "Choices",
            Step::Simulation {..} => "Simulation",
            Step::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Network {..} => network_setup_from_file().is_ok(),
            Step::Waiting => true,
            Step::Accuracy { .. } => true,
            Step::Components { intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator } => {
                *intra != false || *extra != false || *reset != false || *resting != false || *threshold != false || *vmem != false || *tau != false || *ts != false || *adder != false || *multiplier != false || *comparator != false
            },
            Step::FaultType { selection } => { selection.is_some() },
            Step::NumFaults { value, .. } => {
                !value.is_empty() && value.parse::<u64>().is_ok()
            },
            Step::Choices { .. } => true,
            Step::Simulation {..} => true,
            Step::Image { .. } => true,
            Step::End => false,
        }
    }

    fn view(&self) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Network {input_length, hidden_layers_length, output_length, extra_files, intra_files, resting_potential, reset_potential, threshold, beta, tau, spike_length, batch_size, input_spike_train_file}
                => Self::network(input_length, hidden_layers_length, output_length, extra_files, intra_files, resting_potential, reset_potential, threshold, beta, tau, spike_length, batch_size, input_spike_train_file),
            Step::Waiting{} => Self::waiting(),
            Step::Accuracy {snn : _, input_spike_trains: _, targets: _, a} => Self::accuracy(*a),
            Step::Components {intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator }
                => Self::components(*intra, *extra, *reset, *resting, *threshold, *vmem, *tau, *ts, *adder, *multiplier, *comparator),
            Step::FaultType { selection} => Self::fault_type(*selection),
            Step::NumFaults { value} => Self::num_faults(value),
            Step::Choices { c } => {
                Self::choices(c)
            },
            Step::Simulation {a_inj} => Self::simulation((*a_inj).clone()),
            Step::Image { .. } => Self::image(),
            Step::End {} => Self::end(),
        }
            .into()
    }

    fn container(title: &str) -> Column<'a, StepMessage> { //OK
        column![text(title).size(50)].spacing(20)
    }

    fn welcome() -> Column<'a, StepMessage> { //OK
        Self::container("SNN resilience analysis tool")
            .push("Welcome to our simple user interface for studying the resilience of a Spiking Neural Network", )
            .push("Please click Next to select a configuration", )
    }

    fn network(
        input_length: &str, hidden_layers_length: &str, output_length: &str,
        extra_files: &str, intra_files: &str,
        resting_potential: &str, reset_potential: &str, threshold: &str, beta: &str, tau: &str,
        spike_length: &str, batch_size: &str, input_spike_train_file: &str
    ) -> Column<'a, StepMessage> { //OK

        // Read the network parameters from the comfiguration file
        let result = network_setup_from_file();

        // Create the container with the network parameters
        // #to_do: add the possibility to modify the configuration file from the GUI
        // #to_do: create a save changes button
        let c;
        if result.is_ok() {
            let r = result.unwrap();

            let section_a = text(format!("NETWORK DIMENSIONS")).size(20).style(theme::Text::Color(Color::new(0.0, 0.0, 1.0, 1.0)));

            let question1 = text(format!("Input length:  ")).size(20);
            let text_input1: TextInput<'a, StepMessage> = text_input(r.input_layer.to_string().as_str(), input_length)
            .on_input(StepMessage::InputLengthChanged)
            .padding(5)
            .size(20);
            let row1 = row![question1, text_input1];
            
            let question2 = text(format!("Hidden layers length:  ")).size(20);
            let text_input2: TextInput<'a, StepMessage> = text_input(format!("{:?}", r.hidden_layers).as_str().clone(), hidden_layers_length)
            .on_input(StepMessage::HiddenLayersLengthChanged)
            .padding(5)
            .size(20);
            let row2 = row![question2, text_input2];

            let question3 = text(format!("Output length:  ")).size(20);
            let text_input3: TextInput<'a, StepMessage> = text_input(r.output_length.to_string().as_str(), output_length)
            .on_input(StepMessage::OutputLengthChanged)
            .padding(5)
            .size(20);
            let row3 = row![question3, text_input3];

            let section_b = text(format!("WEIGHT FILES")).size(20).style(theme::Text::Color(Color::new(0.0, 0.0, 1.0, 1.0)));

            let question4 = text(format!("Extra weights files:  ")).size(20);
            let text_input4: TextInput<'a, StepMessage> = text_input(format!("{:?}", r.extra_weights).as_str(), extra_files)
            .on_input(StepMessage::ExtraFilesChanged)
            .padding(5)
            .size(20);
            let row4 = row![question4, text_input4];

            let question5 = text(format!("Intra weights files:  ")).size(20);
            let text_input5: TextInput<'a, StepMessage> = text_input(format!("{:?}", r.intra_weights).as_str(), intra_files)
            .on_input(StepMessage::IntraFilesChanged)
            .padding(5)
            .size(20);
            let row5 = row![question5, text_input5];

            let section_c = text(format!("NEURON PARAMETERS")).size(20).style(theme::Text::Color(Color::new(0.0, 0.0, 1.0, 1.0)));

            let question6 = text(format!("Resting potential:  ")).size(20);
            let text_input6: TextInput<'a, StepMessage> = text_input(r.resting_potential.to_string().as_str(), resting_potential)
            .on_input(StepMessage::RestingPotentialChanged)
            .padding(5)
            .size(20);
            let row6 = row![question6, text_input6];

            let question7 = text(format!("Reset potential:  ")).size(20);
            let text_input7: TextInput<'a, StepMessage> = text_input(r.reset_potential.to_string().as_str(), reset_potential)
            .on_input(StepMessage::ResetPotentialChanged)
            .padding(5)
            .size(20);
            let row7 = row![question7, text_input7];

            let question8 = text(format!("Threshold:  ")).size(20);
            let text_input8: TextInput<'a, StepMessage> = text_input(r.threshold.to_string().as_str(), threshold)
            .on_input(StepMessage::ThresholdChanged)
            .padding(5)
            .size(20);
            let row8 = row![question8, text_input8];

            let question9 = text(format!("Beta:  ")).size(20);
            let text_input9: TextInput<'a, StepMessage> = text_input(r.beta.to_string().as_str(), beta)
            .on_input(StepMessage::BetaChanged)
            .padding(5)
            .size(20);
            let row9 = row![question9, text_input9];

            let question10 = text(format!("Tau:  ")).size(20);
            let text_input10: TextInput<'a, StepMessage> = text_input(r.tau.to_string().as_str(), tau)
            .on_input(StepMessage::TauChanged)
            .padding(5)
            .size(20);
            let row10 = row![question10, text_input10];

            let section_d = text(format!("SIMULATION INPUT SPIKES PARAMETERS")).size(20).style(theme::Text::Color(Color::new(0.0, 0.0, 1.0, 1.0)));

            let question11 = text(format!("Spike length:  ")).size(20);
            let text_input11: TextInput<'a, StepMessage> = text_input( r.spike_length.to_string().as_str(), spike_length)
            .on_input(StepMessage::SpikeLengthChanged)
            .padding(5)
            .size(20);
            let row11 = row![question11, text_input11];

            let question12 = text(format!("Batch size:  ")).size(20);
            let text_input12: TextInput<'a, StepMessage> = text_input(r.batch_size.to_string().as_str(), batch_size)
            .on_input(StepMessage::BatchSizeChanged)
            .padding(5)
            .size(20);
            let row12 = row![question12, text_input12];
            
            let question13 = text(format!("Input Spike Train:  ")).size(20);
            let text_input13: TextInput<'a, StepMessage> = text_input(format!("{:?}", r.input_spike_train).as_str(), input_spike_train_file)
            .on_input(StepMessage::InputSpikeTrainFileChanged)
            .padding(5)
            .size(20);
            let row13 = row![question13, text_input13];
            
            c = Self::container("Network configuration")
                .push(section_a)
                .push(row1)
                .push(row2)
                .push(row3)
                .push(section_b)
                .push(row4)
                .push(row5)
                .push(section_c)
                .push(row6)
                .push(row7)
                .push(row8)
                .push(row9)
                .push(row10)
                .push(section_d)
                .push(row11)
                .push(row12)
                .push(row13)
                .push("", )
                .push("Please click Next to build and test the accuracy of your network.", )
                .push("Remeber to click Update to save the changes if you have updated the network's parameters",);
                
        }
        else {
            c = Self::container("Network configuration")
                .push("Please complete the network configuration in file config.toml")
                .push("Please click Update when you have filled up the configuration file.", );
        }
        c
    }

    fn waiting() -> Column<'a, StepMessage> { //OK
        Self::container("We're about to test your network")
            .push("The network accuracy test phase without fault injection takes about 1 minute.")
            .push("Please press Next and wait for the result to appear", )
    }

    fn accuracy(a: f64) -> Column<'a, StepMessage> { //OK
        let question = column![text(format!("The accuracy without faults is: {} %", a)).size(20)];
        Self::container("Network built")
            .push("Your network has been built", )
            .push(question)
            .push("Please click Next to select the configuration of the faults to inject", )
    }

    fn fault_type(selection: Option<FaultType>) -> Column<'a, StepMessage> { //OK
        let question = column![
            text("Select the type of fault").size(20),
            column(FaultType::all().iter().cloned()
                    .map(|fault| { radio(fault,fault,selection,StepMessage::FaultSelected) })
                    .map(Element::from)
                    .collect()
            )
            .spacing(10)
        ].padding(20).spacing(10);

        Self::container("Fault type selection")
            .push(question)
            .push("Please click Next to insert the number of faults to check", )
    }

    fn components( //OK
              intra: bool, extra: bool,
              reset: bool, resting: bool, threshold: bool, vmem: bool, tau: bool, ts: bool,
              adder: bool, multiplier: bool, comparator: bool,
    ) -> Column<'a, StepMessage> {
        let question = column![text("Select in which components you want to insert a fault:").size(20)];
        Self::container("Components selection")
            .push(question)
            .push(checkbox(ComponentType::Intra, intra, StepMessage::IntraSelected ))
            .push(checkbox(ComponentType::Extra, extra, StepMessage::ExtraSelected ))
            .push(checkbox(ComponentType::ResetPotential, reset, StepMessage::RstSelected ))
            .push(checkbox(ComponentType::RestingPotential, resting, StepMessage::RestSelected ))
            .push(checkbox(ComponentType::Threshold, threshold, StepMessage::ThresholdSelected ))
            .push(checkbox(ComponentType::MembranePotential, vmem, StepMessage::MemSelected ))
            .push(checkbox(ComponentType::Tau, tau, StepMessage::TauSelected ))
            .push(checkbox(ComponentType::Ts, ts, StepMessage::TsSelected ))
            .push(checkbox(ComponentType::Adder, adder, StepMessage::AdderSelected ))
            .push(checkbox(ComponentType::Multiplier, multiplier, StepMessage::MulSelected ))
            .push(checkbox(ComponentType::ThresholdComparator, comparator, StepMessage::ComparatorSelected ))
            .push("Please click Next to choose the fault type", )
    }

    fn num_faults(value: &str) -> Column<'a, StepMessage> { //OK
        let question = column![text("Type the number of faults you want to insert:").size(20)];
        let text_input = text_input("Type something to continue...", value)
            .on_input(StepMessage::InputChanged)
            .padding(10)
            .size(30);
        Self::container("Number of faults")
            .push(question)
            .push(text_input)

    }

    fn choices(u: &UserSelection) -> Column<'a, StepMessage> { //OK
        let mut fault = Vec::new();
        fault.push(u.fault_type);
        let mut num = Vec::new();
        num.push(u.num_faults);
        let question = column![
            text("Components selected:").size(20),
            column(u.components.iter().cloned()
                    .map(|c| { checkbox(c, true, StepMessage::IntraSelected ) })
                    .map(Element::from)
                    .collect()
            )
            .spacing(10)
        ].padding(20).spacing(10);
        let question2 = column![
            text("Fault selected:").size(20),
            column(fault.iter().cloned()
                    .map(|c| { checkbox(c, true, StepMessage::IntraSelected ) })
                    .map(Element::from)
                    .collect()
            )
            .spacing(10)
        ].padding(20).spacing(10);
        let question3 = column![
            text("Number of faults introduced:").size(20),
            column(num.iter().cloned()
                    .map(|c| { checkbox(format!("{}", c), true, StepMessage::IntraSelected ) })
                    .map(Element::from)
                    .collect()
            )
            .spacing(10)
        ].padding(20).spacing(10);
        Self::container("Summary of your choices")
            .push(question)
            .push(question2)
            .push(question3)
            .push("Please click Next to run the simulation", )
            .push("This process may take a while. Please wait for the result to appear", )
    }

    fn simulation(a_inj: Vec<(f64, InjectedFault)>) -> Column<'a, StepMessage> { //OK
        let mut questions = Vec::new();
        for ai in a_inj {
            let question = column![text(format!("{}The accuracy with this fault is: {} %", ai.1, ai.0)).size(20)];
            questions.push(question);
        }
        let mut container = Self::container("Simulation finished");
        for question in questions {
            container = container.push(question);
        }
        container
    }
    
    fn image() -> Column<'a, StepMessage> {
        Self::container("Accuracy graphic")
            .push(image("plotters-data/graph.png").width(900))
            .push("Please click Next to see the details of the simulation")
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push("Thank you for using our tool.")
            .push("Please click Restart if you want test other kind of faults on your already built network.", )
    }
}


fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(
        text(label).horizontal_alignment(alignment::Horizontal::Center),
    ).padding(12).width(100)
}

impl From<ComponentType> for String {
    fn from(component: ComponentType) -> String {
        String::from(match component {
            ComponentType::Extra => "Extra weights",
            ComponentType::Intra => "Intra weights",
            ComponentType::ResetPotential => "Reset potential",
            ComponentType::RestingPotential => "Resting potential",
            ComponentType::Threshold => "Threshold",
            ComponentType::MembranePotential => "Membrane potential",
            ComponentType::Tau => "Tau",
            ComponentType::Ts => "Ts",
            ComponentType::Adder => "Adder",
            ComponentType::Multiplier => "Multiplier",
            ComponentType::ThresholdComparator => "Threshold comparator"
        })
    }
}

impl From<FaultType> for String {
    fn from(fault: FaultType) -> String {
        String::from(match fault {
            FaultType::StuckAt0 => "Stuck-at-0",
            FaultType::StuckAt1 => "Stuck-at-1",
            FaultType::TransientBitFlip => "Transient bit flip",
        })
    }
}