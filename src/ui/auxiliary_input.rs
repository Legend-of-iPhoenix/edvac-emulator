use edvac::word::{Word, BIT_WIDTH};
use iced::{Align, Checkbox, Column, Element, Row, Text};

use super::style::text;

// For Maximum Authenticity, this should use small binary toggles but I couldn't
// find these either in their own crate or in iced itself. Implementing a custom
// widget for iced is kinda outside the scope of this project, at least for now.

pub struct AuxiliaryInput {
    bits: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    CheckboxChecked { index: u64, checked: bool },
}

impl AuxiliaryInput {
    pub fn new() -> AuxiliaryInput {
        AuxiliaryInput { bits: 0b0 }
    }

    pub fn update(&mut self, message: Message) -> Word {
        match message {
            // assumption: CheckboxChecked is only fired when checkboxes change.
            Message::CheckboxChecked { index, checked: _ } => {
                self.bits ^= 0b1 << index;
            }
        }

        Word::from_bits(self.bits)
    }

    pub fn view(&mut self) -> Element<Message> {
        let checkboxes = (0_u64..BIT_WIDTH as u64)
            .flat_map(|index| {
                let mut result: Vec<Element<_>> = vec![Column::new()
                    .align_items(Align::Center)
                    .push(
                        Text::new(if index % 3 == 2 || index == BIT_WIDTH as u64 - 1 {
                            format!("{}", 15 - (index / 3))
                        } else {
                            " ".into()
                        })
                        .size(text::SIZE_MEDIUM),
                    )
                    .push(
                        Checkbox::new((self.bits >> index) & 0b1 == 1, "", move |checked: bool| {
                            Message::CheckboxChecked { index, checked }
                        })
                        .spacing(0)
                        .size(text::SIZE_LARGE),
                    )
                    .into()];

                let has_top_tick = index % 3 == 0;

                let has_large_bottom_tick = index % 10 == 3;
                let has_small_bottom_tick = index > 3 && (((index - 3) % 10) % 3 == 0);
                let has_bottom_tick = has_large_bottom_tick | has_small_bottom_tick;

                if has_top_tick || has_bottom_tick {
                    result.push(
                        Column::new()
                            //.height(Length::Shrink)
                            .align_items(Align::Start)
                            .push(
                                Text::new(if has_top_tick { "|" } else { " " })
                                    .size(text::SIZE_MEDIUM), //.height(Length::FillPortion(2)),
                            )
                            .push(Text::new(" ").size(text::SIZE_LARGE))
                            .push(
                                Text::new(if has_bottom_tick { "|" } else { " " })
                                    //.height(Length::FillPortion(1))
                                    .size(if has_large_bottom_tick {
                                        text::SIZE_LARGE
                                    } else {
                                        text::SIZE_MEDIUM
                                    }),
                            )
                            .into(),
                    );
                } else {
                    result.push(Text::new(" ").size(text::SIZE_LARGE).into())
                }

                result
            })
            .rev()
            .collect::<Vec<_>>();
        Column::new()
            .align_items(Align::Center)
            .push(Row::with_children(checkboxes).padding(20).spacing(1))
            .push(Text::new("AUXILIARY INPUT").size(text::SIZE_LARGE))
            .into()
    }
}

impl Default for AuxiliaryInput {
    fn default() -> Self {
        Self::new()
    }
}
