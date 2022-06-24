use edvac::operating_console::MemoryMode;

use iced::{Align, Column, Element, Text};
use iced_audio::{knob, text_marks, tick_marks, IntRange, Knob, Normal, NormalParam};

use super::style::{knob::MemoryModeKnobStyle, text};

pub struct MemoryModeInput {
    range: IntRange,
    tick_marks: tick_marks::Group,
    text_marks: text_marks::Group,

    state: knob::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Input(Normal),
}

impl MemoryModeInput {
    pub fn new() -> MemoryModeInput {
        let mut state = knob::State::new(NormalParam::default());
        state.set_normal((1.0 / 2.0).into());

        MemoryModeInput {
            range: IntRange::new(0, 2),
            tick_marks: tick_marks::Group::evenly_spaced(3, tick_marks::Tier::Two),
            text_marks: text_marks::Group::evenly_spaced(&["L0", "LR", "R1"]),

            state: state,
        }
    }

    pub fn update(&mut self, message: Message) -> MemoryMode {
        match message {
            Message::Input(_) => self.state.snap_visible_to(&self.range),
        }

        [MemoryMode::L0, MemoryMode::LR, MemoryMode::R1][self.state.normal().scale(2.0) as usize]
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .push(
                Knob::new(&mut self.state, Message::Input)
                    .tick_marks(&self.tick_marks)
                    .text_marks(&self.text_marks)
                    .scalar(0.02)
                    .style(MemoryModeKnobStyle),
            )
            .push(Text::new(" ").size(text::SIZE_SMALL)) // for alignment with the operating mode knob
            .push(Text::new("MEMORY BANKS").size(text::SIZE_MEDIUM))
            .into()
    }
}

impl Default for MemoryModeInput {
    fn default() -> Self {
        Self::new()
    }
}
