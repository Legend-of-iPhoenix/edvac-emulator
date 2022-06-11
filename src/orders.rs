use crate::{
    memory::{ADDRESS_MASK, ADDRESS_WIDTH},
    operating_console::ExcessCapacityAction,
    wire::WireShift,
    word::{Word, BIT_WIDTH, U43_MAX},
    Edvac, EdvacStatus,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderKind {
    Compare,
    ManualRead,
    Add,
    Wire,
    Sub,
    Extract,
    Mul,
    MulExact,
    Div,
    DivExact,
    Halt,

    Unused,
}

impl OrderKind {
    #[must_use]
    pub fn from_mneumonic(mneumonic: &str) -> Self {
        match mneumonic {
            "C" => Self::Compare,
            "MR" => Self::ManualRead,
            "A" => Self::Add,
            "W" => Self::Wire,
            "S" => Self::Sub,
            "M" => Self::Mul,
            "m" => Self::MulExact,
            "D" => Self::Div,
            "d" => Self::DivExact,
            "H" => Self::Halt,
            _ => panic!(),
        }
    }
}

impl From<Word> for OrderKind {
    fn from(word: Word) -> Self {
        let bits = word.get_bits();
        // Sources: several- the most useful (I feel) was FuncDesc section 2.6.1
        match (bits & 0b1111) as u8 {
            0b0010 /* +1 */ => OrderKind::Compare,

            // Per Origins+Fate pg. 28-29, the "Visual" order was thrown out and
            // became the "Manual Read" order before or during the construction.
            0b0011 /* -1 */ => OrderKind::ManualRead,

            0b0100 /* +2 */ => OrderKind::Add,
            0b0101 /* -2 */ => OrderKind::Wire,
            0b0110 /* +3 */ => OrderKind::Sub,
            0b0111 /* -3 */ => OrderKind::Extract,
            0b1000 /* +4 */ => OrderKind::Mul,
            0b1001 /* -4 */ => OrderKind::MulExact,
            0b1010 /* +5 */ => OrderKind::Div,
            0b1011 /* -5 */ => OrderKind::DivExact,
            0b1100 /* +6 */ => OrderKind::Halt,

            0b0000 /* +0 */ |
            0b0001 /* -0 */ |
            0b1110 /* +7 */ |
            0b1111 /* -7 */ |
            0b1101 /* -6 */ => OrderKind::Unused,

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Order {
    pub kind: OrderKind,
    pub addresses: [usize; 4],
}

impl From<Word> for Order {
    fn from(word: Word) -> Self {
        let bits = word.get_bits();

        // The order "format" is described in nearly every paper about the EDVAC

        // Perhaps the best illustration can be found on the second page of S.E.
        // Gluck's 1953 paper titled "The Electronic Discrete Variable Computer"
        let addresses = [
            ((bits >> 34) & ADDRESS_MASK) as usize,
            ((bits >> 24) & ADDRESS_MASK) as usize,
            ((bits >> 14) & ADDRESS_MASK) as usize,
            ((bits >> 4) & ADDRESS_MASK) as usize,
        ];

        Order {
            kind: word.into(),
            addresses,
        }
    }
}

impl Edvac {
    fn handle_overflow(&mut self, resume_addr: usize) -> bool {
        println!("Overflow");
        match self.state.excess_capacity_action {
            ExcessCapacityAction::Halt => {
                self.halt(resume_addr);
                false
            }
            ExcessCapacityAction::Ignore => true,
            ExcessCapacityAction::ExecuteSpecial => todo!(),
            ExcessCapacityAction::ExecuteAddressB => todo!(),
        }
    }

    fn execute_compare(
        &mut self,
        a: Word,
        b: Word,
        if_negative: usize,
        if_positive: usize,
    ) -> bool {
        let (difference, _did_overflow) = a.overflowing_sub(b);

        let resume_addr = if difference.is_negative() {
            if_negative
        } else {
            if_positive // or zero
        };

        self.state.initial_address_register = resume_addr;

        false
    }

    fn execute_manual_read(&mut self, dest_a: usize, dest_b: usize, dest_c: usize) -> bool {
        let value = Word::from_bits(self.state.auxiliary_input_switches.read());

        self.set(dest_a, value);
        self.set(dest_b, value);
        self.set(dest_c, value);

        true
    }

    fn execute_add(&mut self, a: Word, b: Word, dest: usize, next_addr: usize) -> bool {
        let (sum, did_overflow) = a.overflowing_add(b);

        self.set(dest, sum);
        if did_overflow {
            self.handle_overflow(next_addr)
        } else {
            true
        }
    }

    fn execute_wire(&mut self, start: usize, sub_order: usize, end: usize) -> bool {
        // Decoding for the sub-order is clearly described in FuncDesc pg "6-16"
        // section 6.3.7
        let backward = ((sub_order >> 9) & 0b1) != 0;

        // See bottom of page "6-17"
        let mut operation = (sub_order >> 6) & 0b011;

        // According to page "6-4" wire #0 is not a wire but a mode of operation
        let wire_spool = sub_order & 0b11;

        if wire_spool == 0 && operation == 0o3 {
            operation = 0o2;
        }

        if backward && operation == 0o3 {
            // halt!
        }

        if wire_spool == 0 && operation == 0o0 {
            // halt!
        }

        // FuncDesc Diagram 104-4LC-3 "Wire Order Selector"
        let mut mem_index = start;
        loop {
            if backward {
                self.translate_wire(wire_spool, WireShift::Backward(BIT_WIDTH));
            }

            match operation {
                0o0 => {
                    // Translate
                }
                0o1 => {
                    // Record (Memory -> Wire)
                    let word = self.get(mem_index);
                    self.write_word_to_wire(wire_spool, word);
                }
                0o2 => {
                    // Read (Wire -> Memory)
                    let word = self.read_word_from_wire(wire_spool);
                    self.set(mem_index, word);
                }
                0o3 => {
                    // Read 5th Addr.
                    mem_index = self.read_address_from_wire(wire_spool);
                    self.translate_wire(wire_spool, WireShift::Forward(ADDRESS_WIDTH));
                    let word = self.read_word_from_wire(wire_spool);
                    self.set(mem_index, word);
                }
                _ => unreachable!(),
            }

            if !backward {
                self.translate_wire(wire_spool, WireShift::Forward(BIT_WIDTH));
            }

            if mem_index == end {
                return true;
            }

            if operation != 0o3 {
                mem_index = (mem_index + 1) & ADDRESS_MASK as usize;
            }
        }
    }

    fn execute_sub(&mut self, a: Word, b: Word, dest: usize, next_addr: usize) -> bool {
        let (difference, did_overflow) = a.overflowing_sub(b);

        self.set(dest, difference);
        if did_overflow {
            self.handle_overflow(next_addr)
        } else {
            true
        }
    }

    fn execute_extract(&mut self, a: Word, shift_code: usize, dest: usize) -> bool {
        let mut a = a.get_bits();
        let stored_sign = a & 0b1;
        a &= !0b1;

        let mut result = self.get(dest).get_bits();

        let sub_order_code = shift_code & 0b111;
        #[allow(clippy::unusual_byte_groupings)]
        let mut shift_amount = (shift_code >> 3) & 0b111_111;
        let shift_direction = (shift_code >> 9) & 0b1; // sanity check

        // see the top of FuncDesc pg. "2-48"
        if shift_amount > 47 {
            shift_amount -= 16;
        }

        let shifted = if shift_direction == 0 {
            a << shift_amount
        } else {
            a >> shift_amount
        };

        println!(
            "            {:0>44b} {} {}",
            shifted, shift_amount, shift_direction
        );

        let mask = match sub_order_code {
            0o1 => ADDRESS_MASK << 34,
            0o2 => ADDRESS_MASK << 24,
            0o3 => ADDRESS_MASK << 14,
            0o4 => ADDRESS_MASK << 4,
            0o5 => 0b1,
            0o6 => U43_MAX << 1,
            0o7 => U43_MAX << 1 | 0b1,
            _ => unreachable!(),
        };

        result = (result & !mask) | (shifted & mask);

        // post-processing/suborder specifics
        if sub_order_code == 0o7 {
            result |= stored_sign;
        }

        self.set(dest, Word::from_bits(result));

        true
    }

    fn execute_mul(&mut self, a: Word, b: Word, dest: usize, exact: bool) -> bool {
        let (rounded, extra_precision) = a.mul(b);

        self.set(dest, rounded);

        if exact {
            self.set((dest + 1) & ADDRESS_MASK as usize, extra_precision);
        }

        true
    }

    fn execute_div(
        &mut self,
        a: Word,
        b: Word,
        dest: usize,
        exact: bool,
        next_addr: usize,
    ) -> bool {
        let (rounded, extra_precision, overflow) = a.overflowing_div(b);

        self.set(dest, rounded);

        if exact {
            self.set((dest + 1) & ADDRESS_MASK as usize, extra_precision);
        }

        if overflow {
            self.handle_overflow(next_addr)
        } else {
            true
        }
    }

    // This is for executing the order `Halt`; `Edvac::halt` is for whenever the
    // machine needs to stop.
    fn execute_halt(&mut self, resume_addr: usize) -> bool {
        self.halt(resume_addr);

        false
    }

    fn halt(&mut self, resume_addr: usize) {
        self.status = EdvacStatus::Halted { resume_addr };
    }

    /// Decodes and executes the *provided* order, returning the next order that
    /// is along the execution path, or None.
    pub fn execute_once(&mut self, order: &Order) -> Option<usize> {
        let [a1, a2, a3, a4] = order.addresses;

        let do_continue = match order.kind {
            OrderKind::Compare => self.execute_compare(self.get(a1), self.get(a2), a3, a4),
            OrderKind::ManualRead => self.execute_manual_read(a1, a2, a3),
            OrderKind::Add => self.execute_add(self.get(a1), self.get(a2), a3, a4),
            OrderKind::Wire => self.execute_wire(a1, a2, a3),
            OrderKind::Sub => self.execute_sub(self.get(a1), self.get(a2), a3, a4),
            OrderKind::Extract => self.execute_extract(self.get(a1), a2, a3),
            OrderKind::Mul => self.execute_mul(self.get(a1), self.get(a2), a3, false),
            OrderKind::MulExact => self.execute_mul(self.get(a1), self.get(a2), a3, true),
            OrderKind::Div => self.execute_div(self.get(a1), self.get(a2), a3, false, a4),
            OrderKind::DivExact => self.execute_div(self.get(a1), self.get(a2), a3, true, a4),
            OrderKind::Halt => self.execute_halt(a4),

            OrderKind::Unused => todo!(),
        };

        if do_continue {
            Some(a4)
        } else {
            None
        }
    }

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
        if let Some(next_address) = self.execute_once(&order) {
            self.state.initial_address_register = next_address;
        }
    }
}
