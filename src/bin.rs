use edvac::{Edvac, EdvacStatus};

mod ui;

use ui::*;

use ui::style::container::ContainerStyle;

use iced::{
    executor, time, Align, Application, Clipboard, Column, Command, Container, Element, Row,
    Settings, Subscription,
};

use std::time::{Duration, Instant};

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

pub struct App {
    computer: Edvac,

    operation_buttons: button_panels::OperationButtons,

    excess_magnitude_options: excess_magnitude_options::ExcessMagnitudeOptions,

    address_a: address_input::AddressInput,
    address_b: address_input::AddressInput,

    special_order: special_order_input::SpecialOrderInput,

    program_loader: program_loader::ProgramLoader,
}

#[derive(Debug, Clone)]
pub enum Message {
    Step(Instant),
    ButtonPressed(button_panels::Message),
    ExcessMagnitudeOptions(excess_magnitude_options::Message),
    AddressA(address_input::Message),
    AddressB(address_input::Message),
    SpecialOrder(special_order_input::Message),
    ProgramLoad(program_loader::Message),
}

impl Application for App {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {
                computer: Edvac::default(),

                operation_buttons: button_panels::OperationButtons::default(),

                excess_magnitude_options: excess_magnitude_options::ExcessMagnitudeOptions::default(
                ),

                address_a: address_input::AddressInput::new("ADDRESS A"),
                address_b: address_input::AddressInput::new("ADDRESS B"),

                special_order: special_order_input::SpecialOrderInput::default(),

                program_loader: program_loader::ProgramLoader::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "EDVAC Emulator".into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Step(_) => {
                self.computer.step_once();
            }
            Message::ButtonPressed(m) => match m {
                button_panels::Message::Initiate => self.computer.initiate_pressed(),
                button_panels::Message::Halt => self.computer.halt_pressed(),

                _ => {} // unimplemented
            },
            Message::ExcessMagnitudeOptions(m) => {
                let (add, div) = self.excess_magnitude_options.update(m);

                self.computer.state.excess_capacity_action_add = add;
                self.computer.state.excess_capacity_action_div = div;
            }
            Message::AddressA(m) => {
                self.computer.state.address_a_switches = self.address_a.update(m);
            }
            Message::AddressB(m) => {
                self.computer.state.address_b_switches = self.address_b.update(m);
            }
            Message::SpecialOrder(m) => {
                self.special_order.update(m);
            }
            Message::ProgramLoad(m) => {
                if let Some((id, wire)) = self.program_loader.update(m) {
                    self.computer.low_speed_memory[id] = wire;
                }
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.computer.status == EdvacStatus::Running {
            time::every(Duration::from_millis(33)).map(Message::Step)
        } else {
            Subscription::none()
        }
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
