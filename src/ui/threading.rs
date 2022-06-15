//! # Allows EDVAC to run in its own thread
use std::thread;

use edvac::{
    operating_console::ExcessCapacityAction,
    wire::{Wire, WireSpool},
    word::Word,
    EdvacStatus,
};

use crate::Edvac;

mod bidi_channel {
    use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender, TryRecvError};

    pub struct BidiChannel<T> {
        tx: Sender<T>,
        rx: Receiver<T>,
    }

    impl<T> BidiChannel<T> {
        pub fn send(&self, message: T) -> Result<(), SendError<T>> {
            self.tx.send(message)
        }

        pub fn recv(&self) -> Result<T, RecvError> {
            self.rx.recv()
        }

        pub fn try_recv(&self) -> Result<T, TryRecvError> {
            self.rx.try_recv()
        }
    }

    pub fn channel_pair<T>() -> (BidiChannel<T>, BidiChannel<T>) {
        let (a_tx, a_rx) = channel();
        let (b_tx, b_rx) = channel();

        (
            BidiChannel { tx: a_tx, rx: b_rx },
            BidiChannel { tx: b_tx, rx: a_rx },
        )
    }
}

pub enum StateParameter {
    ExcessCapacityActions {
        add: ExcessCapacityAction,
        div: ExcessCapacityAction,
    },
    SpecialOrder(Word),

    AddressA(usize),
    AddressB(usize),
}

pub enum EdvacMessage {
    Initiate,
    Halt,

    ModifyState(StateParameter),

    LoadWire(WireSpool, Wire),
}

pub struct EdvacThread {
    channel: bidi_channel::BidiChannel<EdvacMessage>,
}

impl EdvacThread {
    pub fn new() -> EdvacThread {
        let (core_link, ui_link) = bidi_channel::channel_pair();

        thread::spawn(move || {
            let mut computer = Edvac::default();

            fn handle_message(computer: &mut Edvac, message: EdvacMessage) {
                match message {
                    EdvacMessage::Initiate => computer.initiate_pressed(),
                    EdvacMessage::Halt => computer.halt_pressed(),
                    EdvacMessage::ModifyState(parameter) => match parameter {
                        StateParameter::ExcessCapacityActions { add, div } => {
                            computer.state.excess_capacity_action_add = add;
                            computer.state.excess_capacity_action_div = div;
                        }
                        StateParameter::SpecialOrder(word) => {
                            computer.state.special_order_switches = word;
                        }
                        StateParameter::AddressA(address) => {
                            computer.state.address_a_switches = address;
                        }
                        StateParameter::AddressB(address) => {
                            computer.state.address_b_switches = address;
                        }
                    },
                    EdvacMessage::LoadWire(spool, wire) => {
                        computer.low_speed_memory[usize::try_from(spool).unwrap()] = wire;
                    }
                };
            }

            loop {
                if computer.status == EdvacStatus::Running {
                    match core_link.try_recv() {
                        Ok(message) => handle_message(&mut computer, message),
                        Err(std::sync::mpsc::TryRecvError::Empty) => {}
                        Err(_) => todo!(),
                    };

                    computer.step_once();
                } else {
                    match core_link.recv() {
                        Ok(message) => handle_message(&mut computer, message),
                        Err(_) => todo!(),
                    }
                }
            }
        });

        EdvacThread { channel: ui_link }
    }

    pub fn send(&mut self, message: EdvacMessage) {
        self.channel.send(message);
    }
}

impl Default for EdvacThread {
    fn default() -> Self {
        Self::new()
    }
}
