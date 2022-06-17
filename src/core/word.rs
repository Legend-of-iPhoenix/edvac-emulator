use std::ops::Neg;

pub const BIT_WIDTH: usize = 44;
pub const U43_MAX: u64 = 2_u64.pow(43) - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    rep: u64,
    value: i64,
}

impl Word {
    pub fn set_sign(&mut self, negative: bool) {
        if negative {
            self.rep |= 0b1;
            if !self.value.is_negative() {
                self.value = -self.value;
            }
        } else {
            self.rep &= !0b1;
            if !self.value.is_positive() {
                self.value = -self.value;
            }
        }
    }

    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.value < 0
    }

    #[must_use]
    pub fn get_bits(&self) -> u64 {
        self.rep
    }

    pub fn set_bits(&mut self, value: u64) {
        self.rep = value;
        self.value = Word::from_bits_to_i64(value);
    }

    #[must_use]
    pub fn overflowing_add(&self, rhs: Word) -> (Word, bool) {
        let value = self.value + rhs.value;

        if value.unsigned_abs() > U43_MAX {
            ((value - U43_MAX as i64).try_into().unwrap(), true)
        } else {
            (value.try_into().unwrap(), false)
        }
    }

    #[must_use]
    pub fn overflowing_sub(&self, rhs: Word) -> (Word, bool) {
        self.overflowing_add(-rhs)
    }

    #[must_use]
    pub fn mul(&self, rhs: Word) -> (Word, Word) {
        let result = i128::from(self.value) * i128::from(rhs.value);

        let mut most_significant_word: Word = (((result.abs() >> 43) & i128::from(U43_MAX)) as i64)
            .try_into()
            .unwrap();
        let mut least_significant_word: Word = ((result.abs() & i128::from(U43_MAX)) as i64)
            .try_into()
            .unwrap();

        most_significant_word.set_sign(result < 0);
        least_significant_word.set_sign(result < 0);

        (most_significant_word, least_significant_word)
    }

    #[must_use]
    pub fn overflowing_div(&self, rhs: Word) -> (Word, Word, bool) {
        let mut a = i128::from(self.value) << 43;
        let b = i128::from(rhs.value);

        let (most_significant_half, mut overflowed) = a.overflowing_div(b);
        if most_significant_half > i128::from(U43_MAX) {
            overflowed = true;
        }

        a = (a % b) << 43;

        let least_significant_half = a.overflowing_div(b).0;

        let mut most_significant_word: Word = ((most_significant_half.abs() & i128::from(U43_MAX))
            as i64)
            .try_into()
            .unwrap();
        let mut least_significant_word: Word =
            ((least_significant_half.abs() & i128::from(U43_MAX)) as i64)
                .try_into()
                .unwrap();

        most_significant_word.set_sign(most_significant_half < 0);
        least_significant_word.set_sign(most_significant_half < 0);

        (most_significant_word, least_significant_word, overflowed)
    }

    fn from_bits_to_i64(rep: u64) -> i64 {
        let signum: i64 = if rep & 1 == 1 { -1 } else { 1 };
        let bits = (rep >> 1) as i64;

        signum * bits
    }

    #[must_use]
    pub fn from_bits(value: u64) -> Word {
        Word {
            rep: value,
            value: Word::from_bits_to_i64(value),
        }
    }
}

impl Neg for Word {
    type Output = Word;

    fn neg(self) -> Self::Output {
        Word {
            rep: self.rep ^ 0b1,
            value: -self.value,
        }
    }
}

impl TryFrom<i64> for Word {
    // 2's complement 64-bit integer => sign-magnitude 44-bit integer
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let abs = value.unsigned_abs();

        if abs > U43_MAX {
            Err("Value is too large")
        } else {
            Ok(Word {
                rep: (abs << 1) | u64::from(value.is_negative()),
                value,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_i64() {
        const CASES: [(i64, u64); 4] = [
            (1, 0b00000000000000000000000000000000000000000010),
            (-1, 0b00000000000000000000000000000000000000000011),
            (
                U43_MAX as i64,
                0b11111111111111111111111111111111111111111110,
            ),
            (
                -(U43_MAX as i64),
                0b11111111111111111111111111111111111111111111,
            ),
        ];

        for case in CASES {
            let word: Word = case.0.try_into().unwrap();

            assert_eq!(word.get_bits(), case.1, "Conversion from i64 failed");
        }

        for case in CASES {
            let num = Word::from_bits_to_i64(case.1);

            assert_eq!(num, case.0, "Conversion to i64 failed");
        }
    }

    #[test]
    fn mul() {
        const CASES: [((i64, i64), (u64, u64)); 4] = [
            (
                (1, 1),
                (
                    0b00000000000000000000000000000000000000000000,
                    0b00000000000000000000000000000000000000000010,
                ),
            ),
            (
                (-1, -1),
                (
                    0b00000000000000000000000000000000000000000000,
                    0b00000000000000000000000000000000000000000010,
                ),
            ),
            (
                (-1, 1),
                (
                    0b00000000000000000000000000000000000000000001,
                    0b00000000000000000000000000000000000000000011,
                ),
            ),
            (
                (4_294_967_296, 65535),
                (
                    0b00000000000000000000000000000000000000111110,
                    0b11111111111000000000000000000000000000000000,
                ),
            ),
        ];

        for ((multiplicand, multiplier), (most_significant_word, least_significant_word)) in CASES {
            let multiplicand: Word = multiplicand.try_into().unwrap();
            let multiplier: Word = multiplier.try_into().unwrap();

            let result = multiplicand.mul(multiplier);
            assert_eq!(result.0.get_bits(), most_significant_word);
            assert_eq!(result.1.get_bits(), least_significant_word);
        }
    }

    #[test]
    fn overflowing_div() {
        let cases: [((Word, Word), (Word, Word, bool)); 2] = [
            (
                (
                    Word::from_bits(0b00110000000000000000000000000000000000000000),
                    Word::from_bits(0b10100000000000000000000000000000000000000000),
                ),
                (
                    Word::from_bits(0b01001100110011001100110011001100110011001100),
                    Word::from_bits(0b01100110011001100110011001100110011001100110),
                    false,
                ),
            ),
            (
                (
                    Word::from_bits(0b00100000000000000000000000000000000000000000),
                    Word::from_bits(0b10100000000000000000000000000000000000000000),
                ),
                (
                    Word::from_bits(0b00110011001100110011001100110011001100110010),
                    Word::from_bits(0b10011001100110011001100110011001100110011000),
                    false,
                ),
            ),
        ];

        for ((dividend, divisor), result) in cases {
            assert_eq!(dividend.overflowing_div(divisor), result);
        }
    }
}
