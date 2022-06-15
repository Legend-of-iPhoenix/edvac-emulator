use crate::orders::Order;
use crate::wire::{WireShift, WireSpool};
use crate::word::Word;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EdvacStatus {
    Running,
    Halted { resume_addr: usize },
}

impl Default for EdvacStatus {
    fn default() -> Self {
        EdvacStatus::Halted {
            resume_addr: 0o0000,
        }
    }
}

#[derive(Default)]
pub struct Edvac {
    pub state: crate::operating_console::State,
    pub high_speed_memory: crate::high_speed_memory::HighSpeedMemory,

    pub status: EdvacStatus,

    pub low_speed_memory: [crate::wire::Wire; 3],
}

/// # General
impl Edvac {
    pub(crate) fn halt(&mut self, resume_addr: usize) {
        self.status = EdvacStatus::Halted { resume_addr };
    }
}

/// # High-speed memory operations
impl Edvac {
    pub(crate) fn get(&self, addr: usize) -> Word {
        println!(
            "Get {:0>4o}:   {:0>44b} ({})",
            addr,
            self.high_speed_memory
                .get(addr, self.state.memory_mode)
                .get_bits(),
            self.high_speed_memory
                .get(addr, self.state.memory_mode)
                .value,
        );
        self.high_speed_memory.get(addr, self.state.memory_mode)
    }

    pub(crate) fn set(&mut self, addr: usize, val: Word) {
        println!(
            "Set {:0>4o} to {:0>44b}\n        was {:0>44b}",
            addr,
            val.get_bits(),
            self.high_speed_memory
                .get(addr, self.state.memory_mode)
                .get_bits(),
        );

        self.high_speed_memory
            .set(addr, self.state.memory_mode, val);
    }
}

/// # Low-speed Memory operations
impl Edvac {
    pub(crate) fn read_word_from_wire(&mut self, wire_spool: WireSpool) -> Word {
        if let Ok(index) = usize::try_from(wire_spool) {
            println!("Reading word from wire {}", index);

            self.low_speed_memory[index].read_word()
        } else {
            Word::from_bits(self.state.auxiliary_input_switches.get_bits())
        }
    }

    pub(crate) fn read_address_from_wire(&mut self, wire_spool: WireSpool) -> usize {
        if let Ok(index) = usize::try_from(wire_spool) {
            self.low_speed_memory[index].read_address()
        } else {
            unimplemented!()
        }
    }

    pub(crate) fn write_word_to_wire(&mut self, wire_spool: WireSpool, word: Word) {
        if let Ok(index) = usize::try_from(wire_spool) {
            println!("Writing     {:0>44b} to wire {}", word.get_bits(), index);

            self.low_speed_memory[index].write_word(word)
        } else {
            unimplemented!()
        }
    }

    pub(crate) fn translate_wire(&mut self, wire_spool: WireSpool, shift: WireShift) {
        if let Ok(index) = usize::try_from(wire_spool) {
            self.low_speed_memory[index].translate(shift);
        }
        // else condition is omitted- ire zero is treated as if it has "infinite
        // length"
    }
}

/// # Operating Modes
/// Because I only emulate down to the level of individual orders, certain modes
/// like "one cycle" or "one execute" are not supported. *Technically* these can
/// be faked and emulated specifically, but that's not planned, at least for now
impl Edvac {
    /// Decodes and executes the next order, appropriately updating the state of
    /// the machine.
    pub fn step_once(&mut self) {
        println!("======= NEXT CYCLE =======");
        let order: Order = self.get(self.state.initial_address_register).into();

        println!(
            "Ord@{:0>4o}:   {:?} {:0>4o} {:0>4o} {:0>4o} {:0>4o}",
            self.state.initial_address_register,
            order.kind,
            order.addresses[0],
            order.addresses[1],
            order.addresses[2],
            order.addresses[3]
        );

        self.execute_once(&order);
    }

    /// Executes one instruction from the Special Order switches on the front of
    /// the machine.
    pub fn execute_special_order(&mut self) {
        // save current execution address
        let old_address = self.state.initial_address_register;

        self.execute_once(&self.state.special_order_switches.into());

        // the current fourth-address value of the special order instruction was
        // saved to the IAR- the EDVAC does not do this, so we restore the saved
        // execution address from before we ran the order and set the machine to
        // Halt, per FuncDesc pg. "3-9"
        let next_address = self.state.initial_address_register;
        self.state.initial_address_register = old_address;

        self.halt(next_address)
    }

    /// Executes until the Initial Address Register equals Address A (breakpoint
    /// mode).
    ///
    /// Note that this method is provided mostly for completeness; when packaged
    /// as a binary, the UI handles this operating mode. This way, we don't need
    /// to bother with async stuff.
    pub fn continue_to_address_a(&mut self) {
        while self.status == EdvacStatus::Running
            && self.state.initial_address_register != self.state.address_a_switches
        {
            self.step_once();
        }
    }

    /// Executes until the machine Halts.
    ///
    /// Note that this method is provided mostly for completeness; when packaged
    /// as a binary, the UI handles this operating mode. This way, we don't need
    /// to bother with async stuff.
    pub fn continue_to_completion(&mut self) {
        while self.status == EdvacStatus::Running {
            self.step_once();
        }
    }
}

/// # Buttons
impl Edvac {
    pub fn initiate_pressed(&mut self) {
        if let EdvacStatus::Halted { resume_addr } = self.status {
            self.state.initial_address_register = resume_addr;

            self.status = EdvacStatus::Running;
        }

        // todo: the various operating modes that initiate can, well, initiate.
    }

    pub fn halt_pressed(&mut self) {
        self.status = EdvacStatus::Halted {
            resume_addr: self.state.initial_address_register,
        };
    }
}
