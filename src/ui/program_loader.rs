use std::fs;

use iced::{button, Button, Column, Element, Radio, Row, Text};
use rfd::FileDialog;

use edvac::{
    assembler::assemble,
    wire::{Wire, WireSpool},
};
pub struct ProgramLoader {
    button: button::State,
    wire: WireSpool,

    state: State,
}

enum State {
    Error(String),
    Ok,
}

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
    WireSelected(WireSpool),
}

impl ProgramLoader {
    pub fn new() -> ProgramLoader {
        ProgramLoader {
            button: button::State::default(),
            wire: WireSpool::One,
            state: State::Ok,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<(usize, Wire)> {
        match message {
            Message::Clicked => {
                let selection = FileDialog::new()
                    .add_filter("EDVAC program listing", &["edvac"])
                    .pick_file();

                if let Some(path) = selection {
                    if let Ok(listing) = fs::read_to_string(path) {
                        if let Some(wire) = assemble(&listing) {
                            self.state = State::Ok;

                            return Some((self.wire.try_into().unwrap(), wire));
                        } else {
                            self.state = State::Error("Unable to assemble file.".into());
                        }
                    } else {
                        self.state = State::Error("Unable to open file.".into());
                    }
                }

                None
            }
            Message::WireSelected(wire) => {
                self.wire = wire;

                None
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(
                    &mut self.button,
                    Text::new(match &self.state {
                        State::Error(error_str) => error_str,
                        State::Ok => "Load program",
                    }),
                )
                .on_press(Message::Clicked),
            )
            .push(
                [WireSpool::One, WireSpool::Two, WireSpool::Three]
                    .iter()
                    .fold(Column::new(), |column, &variant| {
                        column.push(Radio::new(
                            variant,
                            variant,
                            Some(self.wire),
                            Message::WireSelected,
                        ))
                    }),
            )
            .into()
    }
}

impl Default for ProgramLoader {
    fn default() -> Self {
        Self::new()
    }
}
