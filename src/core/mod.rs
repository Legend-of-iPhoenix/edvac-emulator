pub mod high_speed_memory;
pub mod operating_console;
#[macro_use]
pub mod order_macros;
pub mod orders;
pub mod wire;
pub mod word;

use orders::Order;
use word::Word;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EdvacStatus {
    Running,
    Halted { resume_addr: usize },
}

impl Default for EdvacStatus {
    fn default() -> Self {
        EdvacStatus::Running
            resume_addr: 0o0000,
        }
    }
}

#[derive(Default)]
pub struct Edvac {
    pub state: operating_console::State,
    pub high_speed_memory: high_speed_memory::HighSpeedMemory,

    pub status: EdvacStatus,

    pub low_speed_memory: [wire::Wire; 3],
}

/// # General
impl Edvac {
    fn halt(&mut self, resume_addr: usize) {
        self.status = EdvacStatus::Halted { resume_addr };
    }
}

/// # High-speed memory operations
impl Edvac {
    fn get(&self, addr: usize) -> Word {
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

    fn set(&mut self, addr: usize, val: Word) {
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
    fn read_word_from_wire(&mut self, wire_spool: usize) -> Word {
        if wire_spool == 0 {
            Word::from_bits(self.state.auxiliary_input_switches.get_bits())
        } else {
            assert!((1..=3).contains(&wire_spool));

            println!("Reading word from wire {}", wire_spool);

            self.low_speed_memory[wire_spool - 1].read_word()
        }
    }

    fn read_address_from_wire(&mut self, wire_spool: usize) -> usize {
        assert!((1..=3).contains(&wire_spool));

        self.low_speed_memory[wire_spool - 1].read_address()
    }

    fn write_word_to_wire(&mut self, wire_spool: usize, word: Word) {
        println!(
            "Writing     {:0>44b} to wire {}",
            word.get_bits(),
            wire_spool
        );

        assert!((1..=3).contains(&wire_spool));

        self.low_speed_memory[wire_spool - 1].write_word(word);
    }

    fn translate_wire(&mut self, wire_spool: usize, shift: wire::WireShift) {
        self.low_speed_memory[wire_spool - 1].translate(shift);
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
    pub fn continue_to_address_a(&mut self) {
        while self.status == EdvacStatus::Running
            && self.state.initial_address_register != self.state.address_a_switches
        {
            self.step_once();
        }
    }

    /// Executes until the machine Halts.
    pub fn continue_to_completion(&mut self) {
        while self.status == EdvacStatus::Running {
            self.step_once();
        }
    }
}

/// # Buttons
impl Edvac {
    pub fn initiate(&mut self) {
        if let EdvacStatus::Halted { resume_addr } = self.status {
            self.state.initial_address_register = resume_addr;

            self.status = EdvacStatus::Running;
        }

        // todo: the various operating modes that initiate can, well, initiate.
    }
}
