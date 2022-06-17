use edvac::operating_console::OperatingMode;
use iced::Element;
use iced_audio::{knob, text_marks, tick_marks, Knob, Normal, NormalParam};

use super::style::knob::OperatingModeKnobStyle;

pub struct OperatingModeInput {
    tick_marks: tick_marks::Group,
    text_marks: text_marks::Group,
    state: knob::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(Normal),
}

impl OperatingModeInput {
    pub fn new() -> OperatingModeInput {
        let mut state = knob::State::new(NormalParam::default());
        state.set_normal((2.0 / 8.0).into());

        OperatingModeInput {
            tick_marks: tick_marks::Group::evenly_spaced(8, tick_marks::Tier::Two),
            text_marks: text_marks::Group::evenly_spaced(&[
                "Special - One Cycle",
                "Special - One Execute",
                "Special - One Order",
                "Normal - To Completion",
                "Normal - To Address A",
                "Normal - One Cycle",
                "Normal - One Execute",
                "Normal - One Order",
            ]),

            state,
        }
    }

    pub fn update(&mut self, message: Message) -> OperatingMode {
        match message {
            Message::Input(normal) => {
                let (mode, position) = match normal.scale(8.0) as u8 {
                    0..=2 => (OperatingMode::SpecialOneOrder, 2.0),
                    3 => (OperatingMode::NormalToCompletion, 3.0),
                    4 => (OperatingMode::NormalToAddressA, 4.0),
                    5..=8 => (OperatingMode::NormalOneOrder, 8.0),
                    _ => unreachable!(),
                };

                // snap the visible position while still allowing the knob to be
                // continuous
                self.state.normal_param.value.set(position / 8.0);

                mode
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Knob::new(&mut self.state, Message::Input)
            .tick_marks(&self.tick_marks)
            .text_marks(&self.text_marks)
            .scalar(0.02)
            .style(OperatingModeKnobStyle)
            .into()
    }
}

impl Default for OperatingModeInput {
    fn default() -> Self {
        Self::new()
    }
}
