use iced::{Align, Column, Element, Row, Text};

use iced_audio::{knob, text_marks, tick_marks, IntRange, Knob, Normal};

use super::style::knob::{BinaryKnobStyle, OctalKnobStyle};

pub struct AddressInput {
    bin_range: IntRange,
    // There's no evidence that the original EDVAC had tick marks on these dials
    // I think it looks much nicer with them, so I took some artistic license in
    // adding them in.
    bin_tick_marks: tick_marks::Group,
    bin_text_marks: text_marks::Group,

    bin_state: knob::State,

    octal_range: IntRange,
    // see note above
    octal_tick_marks: tick_marks::Group,
    octal_text_marks: text_marks::Group,

    octal_states: (knob::State, knob::State, knob::State),

    title: Text,
}

#[derive(Debug, Clone)]
pub enum Message {
    Bin(Normal),

    Octal0(Normal),
    Octal1(Normal),
    Octal2(Normal),
}

impl AddressInput {
    pub fn new(title: Text) -> AddressInput {
        let bin_range = IntRange::new(0, 1);
        let octal_range = IntRange::new(0, 7);
        AddressInput {
            bin_range,
            bin_tick_marks: tick_marks::Group::evenly_spaced(2, tick_marks::Tier::Two),
            bin_text_marks: text_marks::Group::evenly_spaced(&["0", "1"]),

            bin_state: knob::State::new(bin_range.default_normal_param()),

            octal_range,
            octal_tick_marks: tick_marks::Group::evenly_spaced(8, tick_marks::Tier::Two),
            octal_text_marks: text_marks::Group::evenly_spaced(&[
                "0", "1", "2", "3", "4", "5", "6", "7",
            ]),

            octal_states: (
                knob::State::new(octal_range.default_normal_param()),
                knob::State::new(octal_range.default_normal_param()),
                knob::State::new(octal_range.default_normal_param()),
            ),

            title,
        }
    }

    pub fn update(&mut self, message: Message) -> usize {
        match message {
            Message::Bin(_) => self.bin_state.snap_visible_to(&self.bin_range),
            Message::Octal0(_) => self.octal_states.0.snap_visible_to(&self.octal_range),
            Message::Octal1(_) => self.octal_states.1.snap_visible_to(&self.octal_range),
            Message::Octal2(_) => self.octal_states.2.snap_visible_to(&self.octal_range),
        }

        let values: (usize, usize, usize, usize) = (
            (self.bin_state.normal().scale(1.0) as usize) & 0b1,
            (self.octal_states.0.normal().scale(7.0) as usize) & 0b111,
            (self.octal_states.1.normal().scale(7.0) as usize) & 0b111,
            (self.octal_states.2.normal().scale(7.0) as usize) & 0b111,
        );

        (values.0 << 9) | (values.1 << 6) | (values.2 << 3) | values.3
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(30)
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .spacing(50)
                    .push(
                        Knob::new(&mut self.bin_state, Message::Bin)
                            .text_marks(&self.bin_text_marks)
                            .tick_marks(&self.bin_tick_marks)
                            .scalar(0.02)
                            .style(BinaryKnobStyle),
                    )
                    .push(
                        Knob::new(&mut self.octal_states.0, Message::Octal0)
                            .text_marks(&self.octal_text_marks)
                            .tick_marks(&self.octal_tick_marks)
                            .scalar(0.015)
                            .style(OctalKnobStyle),
                    )
                    .push(
                        Knob::new(&mut self.octal_states.1, Message::Octal1)
                            .text_marks(&self.octal_text_marks)
                            .tick_marks(&self.octal_tick_marks)
                            .scalar(0.015)
                            .style(OctalKnobStyle),
                    )
                    .push(
                        Knob::new(&mut self.octal_states.2, Message::Octal2)
                            .text_marks(&self.octal_text_marks)
                            .tick_marks(&self.octal_tick_marks)
                            .scalar(0.015)
                            .style(OctalKnobStyle),
                    ),
            )
            .push(self.title.clone())
            .into()
    }
}
