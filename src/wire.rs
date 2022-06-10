use bitvec::prelude::*;

use crate::{
    memory::ADDRESS_WIDTH,
    word::{Word, BIT_WIDTH},
};

const WIRE_SIZE: usize = 50000 * BIT_WIDTH;

pub enum WireShift {
    Forward(usize),
    Backward(usize),
}

pub struct Wire {
    bits: BitBox,
    index: usize,
}

impl Wire {
    pub fn translate(&mut self, shift: WireShift) {
        match shift {
            WireShift::Forward(shift) => {
                self.index += shift;
            }
            WireShift::Backward(shift) => {
                self.index -= shift;
            }
        }
    }

    fn read(&mut self, length: usize) -> Vec<bool> {
        let iter = self.bits.iter().by_vals().skip(self.index);

        iter.take(length).collect()
    }

    pub fn read_word(&mut self) -> Word {
        Word::from_bits(
            self.read(BIT_WIDTH)
                .iter()
                .rev()
                .fold(0, |acc, &bit| acc * 2 + bit as u64),
        )
    }

    pub fn read_address(&mut self) -> usize {
        self.read(ADDRESS_WIDTH)
            .iter()
            .rev()
            .fold(0, |acc, &bit| acc * 2 + bit as usize)
    }

    pub fn write_word(&mut self, word: Word) {
        let mut bits = word.get_bits();

        for i in 0..=BIT_WIDTH {
            let bit = (bits & 0b1) == 0b1;
            bits >>= 1;
            self.bits.set(self.index + i, bit);
        }
    }
}

impl Default for Wire {
    fn default() -> Self {
        Wire {
            bits: bitbox![0; WIRE_SIZE],
            index: 0,
        }
    }
}
