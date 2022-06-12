use edvac::Edvac;

mod ui;

use ui::*;

use iced::{Column, Element, Row, Sandbox, Settings};

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

pub struct App {
    computer: Edvac,

    address_a: address_input::AddressInput,
    address_b: address_input::AddressInput,

    special_order: special_order_input::SpecialOrderInput,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddressA(address_input::Message),
    AddressB(address_input::Message),
    SpecialOrder(special_order_input::Message),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            computer: Edvac::default(),

            address_a: address_input::AddressInput::new("ADDRESS A"),
            address_b: address_input::AddressInput::new("ADDRESS B"),

            special_order: special_order_input::SpecialOrderInput::new(),
        }
    }

    fn title(&self) -> String {
        "EDVAC Emulator".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
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
            .push(
                Row::new()
                    .push(self.address_a.view().map(Message::AddressA))
                    .push(self.address_b.view().map(Message::AddressB)),
            )
            .push(self.special_order.view().map(Message::SpecialOrder))
            .into()
    }
}
