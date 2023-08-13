// possible GUI implementation with iced

use iced::{alignment, Application, Color, executor, Theme, window};
use iced::theme;
use iced::widget::{checkbox, column, container, horizontal_space, radio, row, text, text_input, Button, Column, scrollable};
use iced::{Element, Length, Settings, Command};
use crate::network::snn::SNN;
use crate::resilience::components::ComponentType;
use crate::resilience::fault_models::FaultType;

pub fn launch() -> iced::Result {
    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
    debug: bool,
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
                debug: false,
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
                self.steps.go_back();
                Command::none()
            },
            Message::NextPressed => {
                self.steps.advance();
                if self.steps.current > 1 {
                    for i in 1..self.steps.steps.len() {
                        match self.steps.steps.get(i).unwrap() {
                            Step::Radio { intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator } => {
                                let mut v = Vec::new();
                                if *intra != false { v.push(ComponentType::Intra) }
                                if *extra != false { v.push(ComponentType::Extra) }
                                println!("Componenti selezionate -> {:?}", v);
                            },
                            _ => {}
                        }
                    }
                }
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

        if steps.has_previous() {
            controls = controls.push(button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(button("Next").on_press(Message::NextPressed));
        }

        if steps.is_exit() {
            controls = controls.push(button("Exit").on_press(Message::ExitMessage));
        }

        let content: Element<_> = column![
            steps.view(self.debug).map(Message::StepMessage),
            controls,
        ].max_width(540).spacing(20).padding(20).into();

        let scrollable = scrollable(
            container(if self.debug {
                content.explain(Color::BLACK)
            } else {
                content
            })
                .width(Length::Fill)
                .center_x(),
        );

        container(scrollable).height(Length::Fill).center_y().into()
        //container(content).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
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
                Step::Radio {
                    intra: false, extra: false,
                    reset: false, resting: false, threshold: false, vmem: false, tau: false, ts: false,
                    adder: false, multiplier: false, comparator: false
                },
                Step::Fault { selection: None },
                Step::TextInput { value: String::new() },
                //Step::Image { width: 300 },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(debug)
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
    Fault { selection: Option<FaultType>, },
    Radio {
        intra: bool, extra: bool,
        reset: bool, resting: bool, threshold: bool, vmem: bool, tau: bool, ts: bool,
        adder: bool, multiplier: bool, comparator: bool,
    },
    TextInput { value: String },
    //Image { width: u16, },
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
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
    FaultSelected(FaultType),
    //ImageWidthChanged(u16),
    InputChanged(String),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::IntraSelected(toggle) => {
                if let Step::Radio { intra, .. } = self {
                    *intra = toggle;
                }
            }
            StepMessage::ExtraSelected(toggle) => {
                if let Step::Radio { extra, .. } = self {
                    *extra = toggle;
                }
            }
            StepMessage::RstSelected(toggle) => {
                if let Step::Radio { reset, .. } = self {
                    *reset = toggle;
                }
            }
            StepMessage::RestSelected(toggle) => {
                if let Step::Radio { resting, .. } = self {
                    *resting = toggle;
                }
            }
            StepMessage::ThresholdSelected(toggle) => {
                if let Step::Radio { threshold, .. } = self {
                    *threshold = toggle;
                }
            }
            StepMessage::MemSelected(toggle) => {
                if let Step::Radio { vmem, .. } = self {
                    *vmem = toggle;
                }
            }
            StepMessage::TauSelected(toggle) => {
                if let Step::Radio { tau, .. } = self {
                    *tau = toggle;
                }
            }
            StepMessage::TsSelected(toggle) => {
                if let Step::Radio { ts, .. } = self {
                    *ts = toggle;
                }
            }
            StepMessage::AdderSelected(toggle) => {
                if let Step::Radio { adder, .. } = self {
                    *adder = toggle;
                }
            }
            StepMessage::MulSelected(toggle) => {
                if let Step::Radio { multiplier, .. } = self {
                    *multiplier = toggle;
                }
            }
            StepMessage::ComparatorSelected(toggle) => {
                if let Step::Radio { comparator, .. } = self {
                    *comparator = toggle;
                }
            }
            StepMessage::FaultSelected(sel) => {
                if let Step::Fault { selection } = self {
                    *selection = Some(sel);
                }
            }
            /*StepMessage::ImageWidthChanged(new_width) => {
                if let Step::Image { width, .. } = self {
                    *width = new_width;
                }
            }*/
            StepMessage::InputChanged(new_value) => {
                if let Step::TextInput { value, .. } = self {
                    *value = new_value;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::Radio { .. } => "Components",
            Step::Fault {..} => "Fault",
            //Step::Image { .. } => "Image",
            Step::TextInput { .. } => "Number of faults",
            Step::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Radio { intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator } => {
                *intra != false || *extra != false || *reset != false || *resting != false || *threshold != false || *vmem != false || *tau != false || *ts != false || *adder != false || *multiplier != false || *comparator != false
            },
            Step::Fault { selection } => { selection.is_some() },
            Step::TextInput { value, .. } => {
                !value.is_empty() && value.parse::<i32>().is_ok()
            },
            //Step::Image { .. } => true,
            Step::End => false,
        }
    }

    fn view(&self, _debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Radio {intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator }
                => Self::radio(*intra, *extra, *reset, *resting, *threshold, *vmem, *tau, *ts, *adder, *multiplier, *comparator),
            Step::Fault { selection} => Self::faults(*selection),
            Step::TextInput { value} => Self::num_faults(value),
            //Step::Image { width } => Self::image(*width),
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

    fn faults(selection: Option<FaultType>) -> Column<'a, StepMessage> { //OK
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

    fn radio( //OK
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
        let text_input = text_input("Type something to continue...", value, /* on_change */)
            .on_input(StepMessage::InputChanged)
            .padding(10)
            .size(30);
        Self::container("Number of faults")
            .push(question)
            .push(text_input)

    }
    /*fn image(width: u16) -> Column<'a, StepMessage> {
        Self::container("Image")
            .push("An image that tries to keep its aspect ratio.")
            .push(ferris(width))
            .push(slider(100..=500, width, StepMessage::ImageWidthChanged))
            .push(
                text(format!("Width: {width} px"))
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
    }*/

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
    }
}

/*fn ferris<'a>(width: u16) -> Container<'a, StepMessage> {
    container(
        if cfg!(target_arch = "wasm32") {
            image("tour/images/ferris.png")
        } else {
            image(format!("{}/images/ferris.png", env!("CARGO_MANIFEST_DIR")))
        }
        .width(width),
    )
    .width(Length::Fill)
    .center_x()
}*/

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