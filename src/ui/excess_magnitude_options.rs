use iced::{Align, Column, Element, HorizontalAlignment, Row, Text};
use iced_audio::{knob, text_marks, tick_marks, IntRange, Knob, Normal};

use edvac::operating_console::ExcessCapacityAction;

use super::style::{knob::ExcessMagnitudeKnobStyle, text};

pub struct ExcessMagnitudeOptions {
    range: IntRange,
    tick_marks: tick_marks::Group,
    text_marks: text_marks::Group,

    add_state: knob::State,
    div_state: knob::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Add(Normal),
    Div(Normal),
}

impl ExcessMagnitudeOptions {
    pub fn new() -> ExcessMagnitudeOptions {
        let range = IntRange::new(0, 3);
        ExcessMagnitudeOptions {
            range,
            tick_marks: tick_marks::Group::evenly_spaced(4, tick_marks::Tier::One),
            text_marks: text_marks::Group::evenly_spaced(&[
                "NORMAL",
                "ADDRESS\u{00A0}B",
                "SPECIAL",
                "HALT",
            ]),

            add_state: knob::State::new(range.default_normal_param()),
            div_state: knob::State::new(range.default_normal_param()),
        }
    }

    pub fn update(&mut self, message: Message) -> (ExcessCapacityAction, ExcessCapacityAction) {
        match message {
            Message::Add(_) => self.add_state.snap_visible_to(&self.range),
            Message::Div(_) => self.div_state.snap_visible_to(&self.range),
        }

        const ACTION_MAP: [ExcessCapacityAction; 4] = [
            ExcessCapacityAction::Ignore,
            ExcessCapacityAction::ExecuteAddressB,
            ExcessCapacityAction::ExecuteSpecial,
            ExcessCapacityAction::Halt,
        ];

        (
            ACTION_MAP[self.add_state.normal().scale(3.0) as usize],
            ACTION_MAP[self.div_state.normal().scale(3.0) as usize],
        )
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .spacing(30)
            .align_items(Align::End)
            .push(
                Column::new()
                    .align_items(Align::Center)
                    .padding(30)
                    .push(
                        Knob::new(&mut self.add_state, Message::Add)
                            .tick_marks(&self.tick_marks)
                            .text_marks(&self.text_marks)
                            .scalar(0.02)
                            .style(ExcessMagnitudeKnobStyle),
                    )
                    .push(
                        Row::new()
                            .align_items(Align::End)
                            .push(Text::new("A ").size(text::SIZE_LARGE))
                            .push(Text::new("OR").size(text::SIZE_MEDIUM))
                            .push(Text::new(" S").size(text::SIZE_LARGE)),
                    ),
            )
            .push(
                Text::new("EXCESS MAGNITUDE\nOPTION")
                    .size(text::SIZE_LARGE)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(
                Column::new()
                    .padding(30)
                    .align_items(Align::Center)
                    .push(
                        Knob::new(&mut self.div_state, Message::Div)
                            .tick_marks(&self.tick_marks)
                            .text_marks(&self.text_marks)
                            .scalar(0.02)
                            .style(ExcessMagnitudeKnobStyle),
                    )
                    .push(
                        Row::new()
                            .align_items(Align::End)
                            .push(Text::new("D ").size(text::SIZE_LARGE))
                            .push(Text::new("OR").size(text::SIZE_MEDIUM))
                            .push(Text::new(" d").size(text::SIZE_LARGE)),
                    ),
            )
            .into()
    }
}

impl Default for ExcessMagnitudeOptions {
    fn default() -> Self {
        Self::new()
    }
}
