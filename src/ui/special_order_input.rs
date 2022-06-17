use edvac::orders::OrderKind;
use edvac::word::Word;
use iced_audio::{knob, text_marks, tick_marks, IntRange, Knob, Normal};

use iced::{Align, Column, Element, HorizontalAlignment, Row, Text, VerticalAlignment};

use super::address_input;
use super::style::knob::OrderTypeKnobStyle;

/// In the same order as shown on the dial.
const ORDER_KINDS: [&str; 11] = ["A", "S", "M", "D", "m", "d", "C", "E", "MR", "W", "H"];

pub struct SpecialOrderInput {
    range: IntRange,
    tick_marks: tick_marks::Group,
    text_marks: text_marks::Group,
    order_type_state: knob::State,
    selected_order_kind: OrderKind,

    addresses: [address_input::AddressInput; 4],
    values: [u64; 4],
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
            text_marks: text_marks::Group::evenly_spaced(&ORDER_KINDS),
            order_type_state: knob::State::new(range.default_normal_param()),
            selected_order_kind: OrderKind::Add,

            addresses: [
                address_input::AddressInput::new("ADDRESS 1"),
                address_input::AddressInput::new("ADDRESS 3"),
                address_input::AddressInput::new("ADDRESS 2"),
                address_input::AddressInput::new("ADDRESS 4"),
            ],
            values: [0; 4],
        }
    }

    pub fn update(&mut self, message: Message) -> Word {
        match message {
            Message::OrderType(_) => {
                self.order_type_state.snap_visible_to(&self.range);

                self.selected_order_kind = OrderKind::from_mneumonic(
                    ORDER_KINDS[self.order_type_state.normal().scale(10.0) as usize],
                )
                .unwrap();
            }
            Message::Address(id, m) => {
                self.values[id] = self.addresses[id].update(m) as u64;
            }
        };

        Word::from_bits(
            self.values[0] << 34
                | self.values[2] << 24
                | self.values[1] << 14
                | self.values[3] << 4
                | self.selected_order_kind.to_bits(),
        )
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
