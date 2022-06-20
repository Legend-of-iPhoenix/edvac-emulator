use crate::{operating_console::MemoryMode, word::Word};

// Most of this comes from Origins+Fate pg. 34

pub const ADDRESS_WIDTH: usize = 10;
pub const ADDRESS_MASK: u64 = 2_u64.pow(ADDRESS_WIDTH as u32) - 1;

pub struct HighSpeedMemory {
    bank: [Word; 1024],
}

impl Default for HighSpeedMemory {
    fn default() -> Self {
        HighSpeedMemory {
            bank: [0_i64.try_into().unwrap(); 1024],
        }
    }
}

impl HighSpeedMemory {
    #[must_use]
    pub fn get(&self, addr: usize, mode: MemoryMode) -> Word {
        assert!(addr < 1024);

        match mode {
            MemoryMode::L0 => {
                // Per FuncDesc pg. "5-12"
                // these "special" memory modes enforce a specific tank "without
                // regard to the presence or absense of a pulse at 2^9"
                self.bank[addr % 512]
            }
            MemoryMode::LR => self.bank[addr],
            MemoryMode::R1 => self.bank[(addr % 512) + 512],
        }
    }

    pub fn set(&mut self, addr: usize, mode: MemoryMode, val: Word) {
        assert!(addr < 1024);

        match mode {
            MemoryMode::L0 => {
                self.bank[addr % 512] = val;
            }
            MemoryMode::LR => {
                self.bank[addr] = val;
            }
            MemoryMode::R1 => {
                self.bank[(addr % 512) + 512] = val;
            }
        }
    }

    pub fn load(&mut self, words: Vec<(usize, Word)>) {
        for (addr, word) in words {
            self.set(addr, MemoryMode::default(), word);
        }
    }

    #[must_use]
    pub fn dump(&self) -> [Word; 1024] {
        self.bank
    }
}
