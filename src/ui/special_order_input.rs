use iced_audio::{knob, text_marks, tick_marks, IntRange, Knob, Normal};

use iced::{Align, Column, Element, HorizontalAlignment, Row, Text, VerticalAlignment};

use super::address_input;
use super::style::knob::OrderTypeKnobStyle;

pub struct SpecialOrderInput {
    range: IntRange,
    tick_marks: tick_marks::Group,
    text_marks: text_marks::Group,
    order_type_state: knob::State,

    addresses: [address_input::AddressInput; 4],
}

#[derive(Debug, Clone)]
pub enum Message {
    OrderType(Normal),
    Address(usize, address_input::Message),
}

impl SpecialOrderInput {
    pub fn new() -> SpecialOrderInput {
        let range = IntRange::new(0, 10);
        SpecialOrderInput {
            range,
            tick_marks: tick_marks::Group::evenly_spaced(11, tick_marks::Tier::Two),
            text_marks: text_marks::Group::evenly_spaced(&[
                "A", "S", "M", "D", "m", "d", "C", "E", "MR", "W", "H",
            ]),
            order_type_state: knob::State::new(range.default_normal_param()),

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
            Message::OrderType(_) => self.order_type_state.snap_visible_to(&self.range),
            Message::Address(id, m) => {
                self.addresses[id].update(m);
            }
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
                Column::new()
                    .padding(30)
                    .spacing(50)
                    .align_items(Align::Center)
                    .push(
                        Column::new()
                            .spacing(20)
                            .align_items(Align::Center)
                            .push(
                                Knob::new(&mut self.order_type_state, Message::OrderType)
                                    .tick_marks(&self.tick_marks)
                                    .text_marks(&self.text_marks)
                                    .scalar(0.02)
                                    .style(OrderTypeKnobStyle),
                            )
                            .push(Text::new("TYPE").size(16)),
                    )
                    .push(
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
