use std::fs;

use iced::{button, Button, Column, Element, Radio, Row, Text};
use rfd::FileDialog;

use edvac::{assembler::assemble, wire::Wire};
pub struct ProgramLoader {
    button: button::State,
    wire: WireNumber,

    state: State,
}

enum State {
    Error(String),
    Ok,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum WireNumber {
    One,
    Two,
    Three,
}

impl From<WireNumber> for usize {
    fn from(num: WireNumber) -> Self {
        match num {
            WireNumber::One => 0,
            WireNumber::Two => 1,
            WireNumber::Three => 2,
        }
    }
}

impl From<WireNumber> for String {
    fn from(wire: WireNumber) -> Self {
        format!("Wire {}", usize::from(wire) + 1)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
    WireSelected(WireNumber),
}

impl ProgramLoader {
    pub fn new() -> ProgramLoader {
        ProgramLoader {
            button: button::State::default(),
            wire: WireNumber::One,
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

                            return Some((self.wire.into(), wire));
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
                [WireNumber::One, WireNumber::Two, WireNumber::Three]
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
