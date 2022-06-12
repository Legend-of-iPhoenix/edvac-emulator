use iced::{Column, Element, HorizontalAlignment, Row, Text, VerticalAlignment};

use super::address_input;

pub struct SpecialOrderInput {
    addresses: [address_input::AddressInput; 4],
}

#[derive(Debug, Clone)]
pub enum Message {
    Address(usize, address_input::Message),
}

impl SpecialOrderInput {
    pub fn new() -> SpecialOrderInput {
        SpecialOrderInput {
            addresses: [
                address_input::AddressInput::new("ADDRESS 1"),
                address_input::AddressInput::new("ADDRESS 3"),
                address_input::AddressInput::new("ADDRESS 2"),
                address_input::AddressInput::new("ADDRESS 4"),
            ],
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Address(id, m) => self.addresses[id].update(m),
        };
    }

    pub fn view(&mut self) -> Element<Message> {
        let mut iter = self.addresses.iter_mut();

        Row::new()
            .push(
                Column::new()
                    .push(
                        iter.next()
                            .unwrap()
                            .view()
                            .map(move |message| Message::Address(0, message)),
                    )
                    .push(
                        iter.next()
                            .unwrap()
                            .view()
                            .map(move |message| Message::Address(1, message)),
                    ),
            )
            .push(
                Column::new().push(Text::new("")).push(
                    Text::new("SPECIAL\nORDER")
                        .size(16)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                ),
            )
            .push(
                Column::new()
                    .push(
                        iter.next()
                            .unwrap()
                            .view()
                            .map(move |message| Message::Address(2, message)),
                    )
                    .push(
                        iter.next()
                            .unwrap()
                            .view()
                            .map(move |message| Message::Address(3, message)),
                    ),
            )
            .into()
    }
}

impl Default for SpecialOrderInput {
    fn default() -> Self {
        Self::new()
    }
}
