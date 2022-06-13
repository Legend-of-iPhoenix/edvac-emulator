use iced::{button, Align, Button, Column, Element, Row, Text};

use super::style::button::ButtonStyle;

pub struct OperationButtons {
    clear_button: button::State,
    initiate_button: button::State,
    halt_button: button::State,
    read_out_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Clear,
    Initiate,
    Halt,
    ReadOut,
}

impl OperationButtons {
    pub fn new() -> OperationButtons {
        OperationButtons {
            clear_button: button::State::default(),
            initiate_button: button::State::default(),
            halt_button: button::State::default(),
            read_out_button: button::State::default(),
        }
    }

    // no update method because it's handled by the App logic

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .spacing(20)
                    .push(
                        Column::new()
                            .align_items(Align::Center)
                            .push(Text::new("CLEAR").size(16))
                            .push(
                                Button::new(&mut self.clear_button, Text::new(""))
                                    .padding(20)
                                    .on_press(Message::Clear)
                                    .style(ButtonStyle),
                            ),
                    )
                    .push(
                        Column::new()
                            .align_items(Align::Center)
                            .push(Text::new("INITIATE").size(16))
                            .push(
                                Button::new(&mut self.initiate_button, Text::new(""))
                                    .padding(20)
                                    .on_press(Message::Initiate)
                                    .style(ButtonStyle),
                            ),
                    )
                    .push(
                        Column::new()
                            .align_items(Align::Center)
                            .push(Text::new("HALT").size(16))
                            .push(
                                Button::new(&mut self.halt_button, Text::new(""))
                                    .padding(20)
                                    .on_press(Message::Halt)
                                    .style(ButtonStyle),
                            ),
                    )
                    .push(
                        Column::new()
                            .align_items(Align::Center)
                            .push(Text::new("READ OUT").size(16))
                            .push(
                                Button::new(&mut self.read_out_button, Text::new(""))
                                    .padding(20)
                                    .on_press(Message::ReadOut)
                                    .style(ButtonStyle),
                            ),
                    ),
            )
            .push(Text::new("OPERATION").size(16))
            .into()
    }
}

impl Default for OperationButtons {
    fn default() -> Self {
        Self::new()
    }
}
