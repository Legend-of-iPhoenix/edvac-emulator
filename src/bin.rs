use edvac::Edvac;

mod ui;

use ui::*;

use iced::{Element, Sandbox, Settings};

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

pub struct App {
    _computer: Edvac,

    address_a: address_input::AddressInput,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddressInputMessage(address_input::AddressInputMessage),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            _computer: Edvac::default(),
            address_a: address_input::AddressInput::default(),
        }
    }

    fn title(&self) -> String {
        "EDVAC Emulator".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddressInputMessage(m) => self.address_a.update(m),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.address_a.view().map(Message::AddressInputMessage)
    }
}
