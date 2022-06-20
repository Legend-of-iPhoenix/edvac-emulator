use std::fs;

use iced::{button, Button, Column, Element, Radio, Text};
use rfd::FileDialog;

use edvac::{
    assembler::assemble,
    wire::{Wire, WireSpool},
};

use super::style::text;
pub struct ProgramLoader {
    button: button::State,

    state: State,
}

enum State {
    Ready,
    Message(String),
    Loaded(Wire),
}

#[derive(Debug, Clone)]
pub enum Message {
    Dismiss,
    ProgramSelection,
    WireSelected(WireSpool),
}

impl ProgramLoader {
    pub fn new() -> ProgramLoader {
        ProgramLoader {
            button: button::State::default(),

            state: State::Ready,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<(WireSpool, Wire)> {
        match message {
            Message::Dismiss => {
                self.state = State::Ready;

                None
            }
            Message::ProgramSelection => {
                let selection = FileDialog::new()
                    .add_filter("EDVAC program listing", &["edvac"])
                    .pick_file();

                if let Some(path) = selection {
                    if let Ok(listing) = fs::read_to_string(path) {
                        if let Some(wire) = assemble(&listing) {
                            self.state = State::Loaded(wire);
                        } else {
                            self.state = State::Message("Unable to assemble file.".into());
                        }
                    } else {
                        self.state = State::Message("Unable to open file.".into());
                    }
                }

                None
            }
            Message::WireSelected(spool) => {
                let result = if let State::Loaded(wire) = &self.state {
                    Some((spool, wire.clone()))
                } else {
                    None
                };

                if result.is_some() {
                    self.state = State::Message("File loaded successfully".into());
                } else {
                    // probably unreachable, fail-safe
                    self.state = State::Ready;
                }

                result
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        match &self.state {
            State::Ready => Button::new(
                &mut self.button,
                Text::new("Load Program").size(text::SIZE_MEDIUM),
            )
            .on_press(Message::ProgramSelection)
            .into(),
            State::Message(text) => Column::new()
                .push(Text::new(text.clone()).size(text::SIZE_MEDIUM))
                .push(
                    Button::new(
                        &mut self.button,
                        Text::new("Dismiss").size(text::SIZE_MEDIUM),
                    )
                    .on_press(Message::Dismiss),
                )
                .into(),
            State::Loaded(_) => [WireSpool::One, WireSpool::Two, WireSpool::Three]
                .iter()
                .fold(
                    Column::new()
                        .push(Text::new("Select a wire to load into").size(text::SIZE_MEDIUM)),
                    |column, &variant| {
                        column.push(
                            Radio::new(
                                variant,
                                format!("Wire {}", String::from(variant)),
                                Some(WireSpool::Zero),
                                Message::WireSelected,
                            )
                            .text_size(text::SIZE_MEDIUM),
                        )
                    },
                )
                .into(),
        }
    }
}

impl Default for ProgramLoader {
    fn default() -> Self {
        Self::new()
    }
}
