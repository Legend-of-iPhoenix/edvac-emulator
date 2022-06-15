mod ui;

use ui::*;

use ui::style::container::ContainerStyle;
use ui::threading::{EdvacMessage, StateParameter};

use iced::{Align, Column, Container, Element, Row, Sandbox, Settings};

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

pub struct App {
    computer: threading::EdvacThread,

    operation_buttons: button_panels::OperationButtons,

    excess_magnitude_options: excess_magnitude_options::ExcessMagnitudeOptions,

    address_a: address_input::AddressInput,
    address_b: address_input::AddressInput,

    special_order: special_order_input::SpecialOrderInput,

    program_loader: program_loader::ProgramLoader,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(button_panels::Message),
    ExcessMagnitudeOptions(excess_magnitude_options::Message),
    AddressA(address_input::Message),
    AddressB(address_input::Message),
    SpecialOrder(special_order_input::Message),
    ProgramLoad(program_loader::Message),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            computer: threading::EdvacThread::default(),

            operation_buttons: button_panels::OperationButtons::default(),

            excess_magnitude_options: excess_magnitude_options::ExcessMagnitudeOptions::default(),

            address_a: address_input::AddressInput::new("ADDRESS A"),
            address_b: address_input::AddressInput::new("ADDRESS B"),

            special_order: special_order_input::SpecialOrderInput::default(),

            program_loader: program_loader::ProgramLoader::default(),
        }
    }

    fn title(&self) -> String {
        "EDVAC Emulator".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed(m) => match m {
                button_panels::Message::Initiate => {
                    self.computer.send(EdvacMessage::Initiate);
                }
                button_panels::Message::Halt => {
                    self.computer.send(EdvacMessage::Halt);
                }

                _ => {} // unimplemented
            },
            Message::ExcessMagnitudeOptions(m) => {
                let (add, div) = self.excess_magnitude_options.update(m);

                self.computer.send(EdvacMessage::ModifyState(
                    StateParameter::ExcessCapacityActions { add, div },
                ));
            }
            Message::AddressA(m) => {
                self.computer
                    .send(EdvacMessage::ModifyState(StateParameter::AddressA(
                        self.address_a.update(m),
                    )));
            }
            Message::AddressB(m) => {
                self.computer
                    .send(EdvacMessage::ModifyState(StateParameter::AddressB(
                        self.address_b.update(m),
                    )));
            }
            Message::SpecialOrder(m) => {
                self.computer
                    .send(EdvacMessage::ModifyState(StateParameter::SpecialOrder(
                        self.special_order.update(m),
                    )));
            }
            Message::ProgramLoad(m) => {
                if let Some((spool, wire)) = self.program_loader.update(m) {
                    self.computer.send(EdvacMessage::LoadWire(spool, wire));
                }
            }
        };
    }

    fn view(&mut self) -> Element<Self::Message> {
        Row::new()
            .push(
                Column::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(
                        Row::new().push(
                            Container::new(
                                self.operation_buttons.view().map(Message::ButtonPressed),
                            )
                            .style(ContainerStyle),
                        ),
                    )
                    .push(
                        Container::new(
                            self.excess_magnitude_options
                                .view()
                                .map(Message::ExcessMagnitudeOptions),
                        )
                        .style(ContainerStyle),
                    )
                    .push(
                        Row::new()
                            .spacing(20)
                            .push(
                                Container::new(self.address_a.view().map(Message::AddressA))
                                    .style(ContainerStyle),
                            )
                            .push(
                                Container::new(self.address_b.view().map(Message::AddressB))
                                    .style(ContainerStyle),
                            ),
                    )
                    .push(
                        Container::new(self.special_order.view().map(Message::SpecialOrder))
                            .style(ContainerStyle),
                    ),
            )
            .push(
                Column::new()
                    .spacing(20)
                    .push(self.program_loader.view().map(Message::ProgramLoad)),
            )
            .into()
    }
}
