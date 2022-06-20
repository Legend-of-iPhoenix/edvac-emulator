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
        state.set_normal((6.0 / 9.0).into());

        OperatingModeInput {
            text_marks: text_marks::Group::evenly_spaced(&[
                "",
                "",
                "TO COMPLETION",
                "TO ADDRESS A",
                "ONE CYCLE",
                "ONE EXECUTE",
                "ONE ORDER",
                "",
                "",
                "",
            ]),

            state,
            is_normal: false,
        }
    }

    pub fn update(&mut self, message: Message) -> OperatingMode {
        match message {
            Message::Input(normal) => {
                let (mode, position, is_normal) = match (normal.scale(10.0) as u8, self.is_normal) {
                    (0..=6, false) => (OperatingMode::SpecialOneOrder, 6.0, false),
                    (7.., false) => (OperatingMode::NormalToCompletion, 2.0, true),

                    (0..=1, true) => (OperatingMode::SpecialOneOrder, 6.0, false),
                    (2, true) => (OperatingMode::NormalToCompletion, 2.0, true),
                    (3, true) => (OperatingMode::NormalToAddressA, 3.0, true),
                    (4.., true) => (OperatingMode::NormalOneOrder, 6.0, true),
                };

                // snap the visible position while still allowing the knob to be
                // continuous
                if self.is_normal != is_normal {
                    self.is_normal = is_normal;

                    self.state.set_normal(Normal::new(position / 9.0));
                } else {
                    self.state.normal_param.value.set(position / 9.0);
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
