//! # EDVAC Assembler
//! In addition to the [Rust macro](crate::order_macros) "assembler", this crate
//! provides an assembler that takes a plain-text program listing and puts it on
//! a [Wire] for swapping into the system.
//!
//! ## Assembly Language
//! The assembly language itself is simple enough that it is best explained with
//! an example rather than a formal explanation. Assemblers simply did not exist
//! until after the EDVAC was designed (An EDVAC design document actually led to
//! the creation of the computer that the first assembler was written for!), but
//! I've taken the liberty of including one because it makes writing and storing
//! EDVAC programs convenient on... more modern systems.
//!
//! I modeled the language after a listing of a code for converting numbers from
//! binary-coded decimal to the EDVAC's numerals, as printed in a 1949 ACM paper
//! visible at <https://doi.org/10.2307/2002881>. This program is represented as
//! follows in my EDVAC assembly language:
//! ```text
#![doc = include_str!("../test_programs/dec_to_bin.edvac")]
//! ```
//! ### Notes
//! Lines that don't start with an octal address are treated as comments, as are
//! any extra characters at the end of lines. Leading spaces are ignored in this
//! check.
//!
//! While the above example uses tabs, spaces are also allowed.

use crate::{
    core::wire::Wire,
    high_speed_memory::{ADDRESS_MASK, ADDRESS_WIDTH},
    orders::OrderKind,
    word::Word,
};

// Quick and dirty. Todo: proper error handling, rather than just ignoring lines
pub fn assemble(listing: &str) -> Option<Wire> {
    let assembled = listing
        .lines()
        .filter_map(|line| {
            let mut split = line.split_ascii_whitespace();

            let address = usize::from_str_radix(split.next()?, 8).ok()?;
            if address as u64 & !ADDRESS_MASK != 0 {
                return None;
            }

            let next = split.next()?;
            if ['+', '-'].contains(&next.chars().nth(0)?) {
                let (sign, first) = next.split_at(1);

                let mut raw = u64::from_str_radix(first, 8).ok()?;

                if raw > 0o177 {
                    return None;
                }

                for _fragment_index in 2..=4 {
                    let next_fragment = u64::from_str_radix(split.next()?, 8).ok()?;
                    if next_fragment > 0o7777 {
                        return None;
                    }

                    raw = (raw << 12) | next_fragment;
                }

                Some((
                    address,
                    Word::from_bits((raw << 1) | (if sign == "+" { 0 } else { 1 })),
                ))
            } else {
                // order
                let kind = OrderKind::from_mneumonic(next)?.to_bits();

                let mut raw: u64 = 0;
                for _address_index in 1..=4 {
                    let next_address = u64::from_str_radix(split.next()?, 8).ok()?;
                    if next_address & !ADDRESS_MASK != 0 {
                        return None;
                    }

                    raw = (raw << ADDRESS_WIDTH) | next_address;
                }

                Some((address, Word::from_bits((raw << 4) | kind)))
            }
        })
        .collect::<Vec<_>>();

    Some(Wire::with_program(assembled))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::{wire::WireShift, word::BIT_WIDTH};

    #[test]
    fn assemble_doc() {
        #[rustfmt::skip]
        let words: [(usize, Word); 45] = [
            (0o0000, Word::from_bits(0b10000000010000000011100000001110000000110111)),
            (0o1003, Word::from_bits(0b00000000010010000010000000000010000000100101)),
            (0o1002, Word::from_bits(0b10000000110010100001100000011010000001110111)),
            (0o1007, Word::from_bits(0b10000000110000000011100010110110000001100111)),
            (0o1006, Word::from_bits(0b00000000000000000111100010110010000001010111)),
            (0o1005, Word::from_bits(0b10000001101010100011100010100010000010010111)),
            (0o1011, Word::from_bits(0b10001011000100100111100010011110000001000111)),
            (0o1004, Word::from_bits(0b10000011101000100111100010101010000010111000)),
            (0o1013, Word::from_bits(0b10001011000100000111100010111010000010000111)),
            (0o1010, Word::from_bits(0b10001010101000101110100010011110000010100100)),
            (0o1012, Word::from_bits(0b10000011101000100111100010101010000100111000)),
            (0o1023, Word::from_bits(0b10001011000011100111100010111010000011110111)),
            (0o1017, Word::from_bits(0b10001010101000101110100010011110000011000100)),
            (0o1014, Word::from_bits(0b10000011101000100111100010101010000110111000)),
            (0o1033, Word::from_bits(0b10001011000011000111100010111010000100000111)),
            (0o1020, Word::from_bits(0b10001010101000101110100010011110000011010100)),
            (0o1015, Word::from_bits(0b10000011101000100111100010101010001000111000)),
            (0o1043, Word::from_bits(0b10001011000010100111100010111010000100010111)),
            (0o1021, Word::from_bits(0b10001010101000101110100010011110000100100100)),
            (0o1022, Word::from_bits(0b10000011101000100111100010101010001010111000)),
            (0o1053, Word::from_bits(0b10001011000010000111100010111010000101110111)),
            (0o1027, Word::from_bits(0b10001010101000101110100010011110000101000100)),
            (0o1024, Word::from_bits(0b10000011101000100111100010101010001100111000)),
            (0o1063, Word::from_bits(0b10001011000001100111100010111010000110000111)),
            (0o1030, Word::from_bits(0b10001010101000101110100010011110000101010100)),
            (0o1025, Word::from_bits(0b10000011101000100111100010101010001110111000)),
            (0o1073, Word::from_bits(0b10001011000001000111100010111010000110010111)),
            (0o1031, Word::from_bits(0b10001010101000101110100010011110000110100100)),
            (0o1032, Word::from_bits(0b10000011101000100111100010101010010000111000)),
            (0o1103, Word::from_bits(0b10001011000000100111100010111010001000010111)),
            (0o1041, Word::from_bits(0b10001010101000101110100010011110001010010100)),
            (0o1051, Word::from_bits(0b10000101101000100111100010101010010010111000)),
            (0o1113, Word::from_bits(0b10001011001000101010100010010010001010000100)),
            (0o1050, Word::from_bits(0b10001001001000011101000000000010001001011010)),
            (0o1045, Word::from_bits(0b10000001101000100010100000011010000111110110)),
            (0o1037, Word::from_bits(0b10000001101000100010100010110110000001100010)),
            (0o1055, Word::from_bits(0b00000000010001000011000000000010001000000101)),
            (0o1040, Word::from_bits(0b10000000011000100010100001110010000111100010)),
            (0o1036, Word::from_bits(0b10000000011000100010100000000110001001100110)),
            (0o1046, Word::from_bits(0b10001000101001011011100000001110000000110111)),
            (0o1034, Word::from_bits(0b00000000000000000000000000000000000000001100)),
            (0o1016, Word::from_bits(0b00011001100110011001100110011001100110011010)),
            (0o1026, Word::from_bits(0b00001001100110011001100110011001100110011010)),
            (0o1035, Word::from_bits(0b10100000000000000000000000000000000000000000)),
            (0o1042, Word::from_bits(0b00000000010000000000000000000000000000000000)),
        ];

        let listing = include_str!("../test_programs/dec_to_bin.edvac");

        let mut wire = assemble(listing).unwrap();

        for (address, word) in words {
            assert_eq!(wire.read_address(), address);
            wire.translate(WireShift::Forward(ADDRESS_WIDTH));
            assert_eq!(wire.read_word(), word);
            wire.translate(WireShift::Forward(BIT_WIDTH));
        }
    }
}
