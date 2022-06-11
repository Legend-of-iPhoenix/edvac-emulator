use iced::{Element, Row};

use iced_audio::{
    knob, text_marks, tick_marks, IntRange, Knob, Normal,
};

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
}

#[derive(Debug, Clone)]
pub enum AddressInputMessage {
    BinChanged(Normal),

    Octal0Changed(Normal),
    Octal1Changed(Normal),
    Octal2Changed(Normal),
}

impl AddressInput {
    pub fn new() -> AddressInput {
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
        }
    }

    pub fn update(&mut self, message: AddressInputMessage) {
        match message {
            AddressInputMessage::BinChanged(_) => self.bin_state.snap_visible_to(&self.bin_range),
            AddressInputMessage::Octal0Changed(_) => {
                self.octal_states.0.snap_visible_to(&self.octal_range)
            }
            AddressInputMessage::Octal1Changed(_) => {
                self.octal_states.1.snap_visible_to(&self.octal_range)
            }
            AddressInputMessage::Octal2Changed(_) => {
                self.octal_states.2.snap_visible_to(&self.octal_range)
            }
        }
    }

    pub fn view(&mut self) -> Element<AddressInputMessage> {
        Row::new()
            .spacing(50)
            .padding(50)
            .push(
                Knob::new(&mut self.bin_state, AddressInputMessage::BinChanged)
                    .text_marks(&self.bin_text_marks)
                    .tick_marks(&self.bin_tick_marks)
                    .scalar(0.02)
                    .style(BinaryKnobStyle),
            )
            .push(
                Knob::new(&mut self.octal_states.0, AddressInputMessage::Octal0Changed)
                    .text_marks(&self.octal_text_marks)
                    .tick_marks(&self.octal_tick_marks)
                    .scalar(0.015)
                    .style(OctalKnobStyle),
            )
            .push(
                Knob::new(&mut self.octal_states.1, AddressInputMessage::Octal1Changed)
                    .text_marks(&self.octal_text_marks)
                    .tick_marks(&self.octal_tick_marks)
                    .scalar(0.015)
                    .style(OctalKnobStyle),
            )
            .push(
                Knob::new(&mut self.octal_states.2, AddressInputMessage::Octal2Changed)
                    .text_marks(&self.octal_text_marks)
                    .tick_marks(&self.octal_tick_marks)
                    .scalar(0.015)
                    .style(OctalKnobStyle),
            )
            .into()
    }
}

impl Default for AddressInput {
    fn default() -> Self {
        Self::new()
    }
}
