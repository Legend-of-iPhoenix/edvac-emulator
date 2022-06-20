use edvac::word::{Word, BIT_WIDTH};
use iced::{Checkbox, Element, Row};

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
            .map(|index| {
                Checkbox::new((self.bits >> index) & 0b1 == 1, "", move |checked: bool| {
                    Message::CheckboxChecked { index, checked }
                })
                .spacing(0)
                .size(16)
                .into()
            })
            .rev()
            .collect::<Vec<_>>();
        Row::with_children(checkboxes).padding(20).spacing(1).into()
    }
}

impl Default for AuxiliaryInput {
    fn default() -> Self {
        Self::new()
    }
}
