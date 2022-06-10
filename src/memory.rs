use crate::{operating_console::MemoryMode, word::Word};

// Most of this comes from Origins+Fate pg. 34

pub const ADDRESS_WIDTH: usize = 10;
pub const ADDRESS_MASK: u64 = 0b1111111111; // ten bits

pub struct Memory {
    bank: [Word; 1024],
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            bank: [0_i64.try_into().unwrap(); 1024],
        }
    }
}

impl Memory {
    pub fn get(&self, addr: usize, mode: MemoryMode) -> Word {
        match mode {
            MemoryMode::L0 => {
                assert!(addr < 512);

                self.bank[addr]
            }
            MemoryMode::LR => {
                assert!(addr < 1024);

                self.bank[addr]
            }
            MemoryMode::R1 => {
                assert!(addr < 512);

                self.bank[addr + 512]
            }
        }
    }

    pub fn set(&mut self, addr: usize, mode: MemoryMode, val: Word) {
        match mode {
            MemoryMode::L0 => {
                assert!(addr < 512);

                self.bank[addr] = val;
            }
            MemoryMode::LR => {
                assert!(addr < 1024);

                self.bank[addr] = val;
            }
            MemoryMode::R1 => {
                assert!(addr < 512);

                self.bank[addr + 512] = val;
            }
        }
    }

    #[cfg(test)]
    pub fn load(&mut self, words: Vec<(usize, Word)>) {
        for (addr, word) in words {
            self.set(addr, MemoryMode::default(), word);
        }
    }

    #[cfg(test)]
    pub fn dump(&self) -> [Word; 1024] {
        self.bank
    }
}
