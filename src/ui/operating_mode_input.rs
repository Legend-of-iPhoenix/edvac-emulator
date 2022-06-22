use edvac::operating_console::OperatingMode;
use iced::{Align, Column, Element, Text};
use iced_audio::{knob, text_marks, Knob, Normal, NormalParam};

use super::style::{knob::OperatingModeKnobStyle, text};

pub struct OperatingModeInput {
    text_marks: text_marks::Group,
    state: knob::State,
    is_normal: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(Normal),
}

impl OperatingModeInput {
    pub fn new() -> OperatingModeInput {
        let mut state = knob::State::new(NormalParam::default());
        state.set_normal((5.0 / 6.0).into());

        OperatingModeInput {
            text_marks: text_marks::Group::evenly_spaced(&[
                "",
                "TO COMPLETION",
                "TO ADDRESS A",
                "ONE CYCLE",
                "ONE EXECUTE",
                "ONE ORDER",
                "",
            ]),

            state,
            is_normal: false,
        }
    }

    pub fn update(&mut self, message: Message) -> OperatingMode {
        match message {
            Message::Input(normal) => {
                let (mode, position, is_normal) = match (normal.scale(7.0) as u8, self.is_normal) {
                    (0..=5, false) => (OperatingMode::SpecialOneOrder, 5.0, false),
                    (5.., false) => (OperatingMode::NormalToCompletion, 1.0, true),

                    (0, true) => (OperatingMode::SpecialOneOrder, 5.0, false),
                    (1, true) => (OperatingMode::NormalToCompletion, 1.0, true),
                    (2, true) => (OperatingMode::NormalToAddressA, 2.0, true),
                    (3.., true) => (OperatingMode::NormalOneOrder, 5.0, true),
                };

                // snap the visible position while still allowing the knob to be
                // continuous
                if self.is_normal != is_normal {
                    self.is_normal = is_normal;

                    self.state.set_normal(Normal::new(position / 6.0));
                } else {
                    self.state.normal_param.value.set(position / 6.0);
                }

                mode
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .push(
                Knob::new(&mut self.state, Message::Input)
                    .text_marks(&self.text_marks)
                    .scalar(0.02)
                    .style(OperatingModeKnobStyle),
            )
            .push(
                Text::new(if self.is_normal { "NORMAL" } else { "SPECIAL" }).size(text::SIZE_SMALL),
            )
            .push(Text::new("MODE OF OPERATION").size(text::SIZE_MEDIUM))
            .into()
    }
}

impl Default for OperatingModeInput {
    fn default() -> Self {
        Self::new()
    }
}
