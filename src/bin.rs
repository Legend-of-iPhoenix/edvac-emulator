use edvac::Edvac;

mod ui;

use ui::*;

use ui::style::container::ContainerStyle;

use iced::{Align, Column, Container, Element, Row, Sandbox, Settings};

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

pub struct App {
    computer: Edvac,

    excess_magnitude_options: excess_magnitude_options::ExcessMagnitudeOptions,

    address_a: address_input::AddressInput,
    address_b: address_input::AddressInput,

    special_order: special_order_input::SpecialOrderInput,
}

#[derive(Debug, Clone)]
pub enum Message {
    ExcessMagnitudeOptions(excess_magnitude_options::Message),
    AddressA(address_input::Message),
    AddressB(address_input::Message),
    SpecialOrder(special_order_input::Message),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            computer: Edvac::default(),

            excess_magnitude_options: excess_magnitude_options::ExcessMagnitudeOptions::default(),

            address_a: address_input::AddressInput::new("ADDRESS A"),
            address_b: address_input::AddressInput::new("ADDRESS B"),

            special_order: special_order_input::SpecialOrderInput::default(),
        }
    }

    fn title(&self) -> String {
        "EDVAC Emulator".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
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
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .spacing(20)
            .align_items(Align::Center)
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
            )
            .into()
    }
}
