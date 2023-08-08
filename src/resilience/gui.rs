
// possible GUI implementation with iced
use iced::{alignment};
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row,
    scrollable, slider, text, text_input,
};
use iced::widget::{Button, Column, Container};
use iced::{Color, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    env_logger::init();
    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
    debug: bool,
}

impl Sandbox for Tour {
    type Message = Message;

    fn new() -> Tour {
        Tour {
            steps: Steps::new(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Tour { steps, .. } = self;

        let mut controls = row![];

        if steps.has_previous() {
            controls = controls.push(
                button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls =
                controls.push(button("Next").on_press(Message::NextPressed));
        }

        let content: Element<_> = column![
            steps.view(self.debug).map(Message::StepMessage),
            controls,
        ]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

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
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}


#[derive(Debug, Clone, Copy)]
pub enum MessageExit {
    Confirm,
    Exit,
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Radio {
                    intra: false,
                    extra: false,
                    reset: false,
                    resting: false,
                    threshold: false,
                    vmem: false,
                    tau: false,
                    ts: false,
                    adder: false,
                    multiplier: false,
                    comparator: false
                },
                Step::Fault { selection: None },
                Step::TextInput {
                    value: String::new(),
                },
                //Step::Image { width: 300 },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&self, _debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(_debug)
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

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    Welcome,
    Fault { selection: Option<Faults>, },
    Radio {
        intra: bool,
        extra: bool,
        reset: bool,
        resting: bool,
        threshold: bool,
        vmem: bool,
        tau: bool,
        ts: bool,
        adder: bool,
        multiplier: bool,
        comparator: bool,
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
    FaultSelected(Faults),
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
            StepMessage::FaultSelected(language) => {
                if let Step::Fault { selection } = self {
                    *selection = Some(language);
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
            Step::Radio { .. } => "Radio button",
            Step::Fault {..} => "Fault",
            //Step::Image { .. } => "Image",
            Step::TextInput { .. } => "Text input",
            Step::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Radio { .. } => true,
            Step::Fault { .. } => true,
            Step::TextInput { value, .. } => !value.is_empty(),
            //Step::Image { .. } => true,
            Step::End => false,
        }
    }

    fn view(&self, _debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Radio {intra,extra,reset,resting, threshold, vmem, tau, ts, adder, multiplier, comparator }
            => Self::radio(*intra, *extra, *reset, *resting, *threshold, *vmem, *tau, *ts, *adder, *multiplier, *comparator),
            Step::Fault {selection} => Self::faults(*selection),
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
        Self::container("Welcome!")
            .push("This is a simple interface for the user to study the resilience of a Spiking Neural Network", )
            .push("Please click the Next bottom to choose the configuration", )
    }

    fn faults(selection: Option<Faults>) -> Column<'a, StepMessage> { //OK
        let question = column![
            text("Select the type of fault").size(20),
            column(Faults::all().iter().cloned()
                    .map(|fault| { radio(fault,fault,selection,StepMessage::FaultSelected) })
                    .map(Element::from)
                    .collect()
            )
            .spacing(10)
        ].padding(20).spacing(10);

        Self::container("Fault type selection")
            .push(question)
            .push("Please click the Next button to insert the number of faults to check", )
    }

    fn radio( //OK
              intra: bool, extra: bool,
              reset: bool, resting: bool, threshold: bool, vmem: bool, tau: bool, ts: bool,
              adder: bool, multiplier: bool, comparator: bool,
    ) -> Column<'a, StepMessage> {
        let question = column![text("Select in which components you want to insert a fault:").size(20)];
        Self::container("Components selection")
            .push(question)
            .push(checkbox(Components::IntraWeights, intra, StepMessage::IntraSelected, ))
            .push(checkbox(Components::ExtraWeights, extra, StepMessage::ExtraSelected, ))
            .push(checkbox(Components::ResetPotential, reset, StepMessage::RstSelected, ))
            .push(checkbox(Components::RestingPotential, resting, StepMessage::RestSelected, ))
            .push(checkbox(Components::Threshold, threshold, StepMessage::ThresholdSelected, ))
            .push(checkbox(Components::MembranePotential, vmem, StepMessage::MemSelected, ))
            .push(checkbox(Components::Tau, tau, StepMessage::TauSelected, ))
            .push(checkbox(Components::Ts, ts, StepMessage::TsSelected, ))
            .push(checkbox(Components::Adder, adder, StepMessage::AdderSelected, ))
            .push(checkbox(Components::Multiplier, multiplier, StepMessage::MulSelected, ))
            .push(checkbox(Components::ThresholdComparison, comparator, StepMessage::ComparatorSelected, ))
            .push("Please click the Next bottom to choose the fault type", )
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
    )
        .padding(12)
        .width(100)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    ExtraWeights,
    IntraWeights,
    ResetPotential,
    RestingPotential,
    Threshold,
    MembranePotential,
    Tau,
    Ts,
    Adder,
    Multiplier,
    ThresholdComparison,
}

impl From<Components> for String {
    fn from(component: Components) -> String {
        String::from(match component {
            Components::ExtraWeights => "Extra weights",
            Components::IntraWeights => "Intra weights",
            Components::ResetPotential => "Reset potential",
            Components::RestingPotential => "Resting potential",
            Components::Threshold => "Threshold",
            Components::MembranePotential => "Membrane potential",
            Components::Tau => "Tau",
            Components::Ts => "Ts",
            Components::Adder => "Adder",
            Components::Multiplier => "Multiplier",
            Components::ThresholdComparison => "Threshold comparison"
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faults {
    Stuck0,
    Stuck1,
    BitFlip
}

impl Faults {
    fn all() -> [Faults; 3] {
        [
            Faults::Stuck0,
            Faults::Stuck1,
            Faults::BitFlip
        ]
    }
}

impl From<Faults> for String {
    fn from(fault: Faults) -> String {
        String::from(match fault {
            Faults::Stuck0 => "Stuck-at-0",
            Faults::Stuck1 => "Stuck-at-1",
            Faults::BitFlip => "Bit flip",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}
