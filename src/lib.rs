use crate::word::Word;
use wire::WireShift;

pub mod memory;
pub mod operating_console;
#[macro_use]
pub mod order_macros;
pub mod orders;
pub mod wire;
pub mod word;

#[cfg(test)]
pub mod test_programs;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EdvacStatus {
    Running,
    Halted { resume_addr: usize },
}

impl Default for EdvacStatus {
    fn default() -> Self {
        EdvacStatus::Running
    }
}

pub struct Edvac {
    pub state: operating_console::State,
    pub memory: memory::Memory,

    pub status: EdvacStatus,

    pub low_speed_memory: [Box<wire::Wire>; 3],
}

impl Edvac {
    fn get(&self, addr: usize) -> Word {
        println!(
            "Get {:0>4o}:   {:0>44b} ({})",
            addr,
            self.memory.get(addr, self.state.memory_mode).get_bits(),
            self.memory.get(addr, self.state.memory_mode).value,
        );
        self.memory.get(addr, self.state.memory_mode)
    }

    fn set(&mut self, addr: usize, val: Word) {
        println!(
            "Set {:0>4o} to {:0>44b}\n        was {:0>44b}",
            addr,
            val.get_bits(),
            self.memory.get(addr, self.state.memory_mode).get_bits(),
        );
        self.memory.set(addr, self.state.memory_mode, val)
    }

    fn read_word_from_wire(&mut self, wire_spool: usize) -> Word {
        if wire_spool == 0 {
            Word::from_bits(self.state.auxiliary_input_switches.read())
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

        self.low_speed_memory[wire_spool - 1].write_word(word)
    }

    fn translate_wire(&mut self, wire_spool: usize, shift: WireShift) {
        self.low_speed_memory[wire_spool - 1].translate(shift);
    }
}
