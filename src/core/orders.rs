use crate::{
    high_speed_memory::{ADDRESS_MASK, ADDRESS_WIDTH},
    operating_console::ExcessCapacityAction,
    wire::WireShift,
    word::{Word, BIT_WIDTH, U43_MAX},
    Edvac,
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
    pub fn from_mneumonic(mneumonic: &str) -> Option<Self> {
        match mneumonic {
            "C" => Some(Self::Compare),
            "MR" => Some(Self::ManualRead),
            "A" => Some(Self::Add),
            "W" => Some(Self::Wire),
            "S" => Some(Self::Sub),
            "E" => Some(Self::Extract),
            "M" => Some(Self::Mul),
            "m" => Some(Self::MulExact),
            "D" => Some(Self::Div),
            "d" => Some(Self::DivExact),
            "H" => Some(Self::Halt),
            _ => None,
        }
    }

    #[must_use]
    pub fn to_bits(self) -> u64 {
        // Opposite of the From<Word> operation below, see comments there.
        match self {
            OrderKind::Compare => 0b0010,
            OrderKind::ManualRead => 0b0011,
            OrderKind::Add => 0b0100,
            OrderKind::Wire => 0b0101,
            OrderKind::Sub => 0b0110,
            OrderKind::Extract => 0b0111,
            OrderKind::Mul => 0b1000,
            OrderKind::MulExact => 0b1001,
            OrderKind::Div => 0b1010,
            OrderKind::DivExact => 0b1011,
            OrderKind::Halt => 0b1100,
            OrderKind::Unused => 0b0000,
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
    fn handle_overflow(&mut self, is_div: bool, resume_addr: usize) {
        println!("Overflow");

        let action = if is_div {
            self.state.excess_capacity_action_div
        } else {
            self.state.excess_capacity_action_add
        };

        match action {
            ExcessCapacityAction::Halt => {
                self.halt(resume_addr);
            }
            ExcessCapacityAction::Ignore => {
                self.state.initial_address_register = resume_addr;
            }
            ExcessCapacityAction::ExecuteSpecial => {
                self.execute_once(&self.state.special_order_switches.into());
                self.state.initial_address_register = resume_addr; // overwrite!
            }
            ExcessCapacityAction::ExecuteAddressB => {
                self.execute_once(&self.get(self.state.address_b_switches).into());
                self.state.initial_address_register = resume_addr; // overwrite!
            }
        }
    }

    fn execute_compare(&mut self, addresses: [usize; 4]) {
        let a = self.get(addresses[0]);
        let b = self.get(addresses[1]);
        let (difference, _did_overflow) = a.overflowing_sub(b);

        let resume_addr = if difference.is_negative() {
            addresses[2]
        } else {
            addresses[3] // positive or zero
        };

        self.state.initial_address_register = resume_addr;
    }

    fn execute_manual_read(&mut self, addresses: [usize; 4]) {
        let value = Word::from_bits(self.state.auxiliary_input_switches.get_bits());

        self.set(addresses[0], value);
        self.set(addresses[1], value);
        self.set(addresses[2], value);

        self.state.initial_address_register = addresses[3];
    }

    fn execute_add(&mut self, addresses: [usize; 4]) {
        let a = self.get(addresses[0]);
        let b = self.get(addresses[1]);
        let (sum, did_overflow) = a.overflowing_add(b);

        self.set(addresses[2], sum);
        if did_overflow {
            self.handle_overflow(false, addresses[3]);
        } else {
            self.state.initial_address_register = addresses[3];
        }
    }

    fn execute_wire(&mut self, addresses: [usize; 4]) {
        let start = addresses[0];
        let sub_order = addresses[1];
        let end = addresses[2];

        let next_addr = addresses[3];
        // Decoding for the sub-order is clearly described in FuncDesc pg "6-16"
        // section 6.3.7
        let backward = ((sub_order >> 9) & 0b1) != 0;

        // See bottom of page "6-17"
        let mut operation = (sub_order >> 6) & 0b011;

        // According to page "6-4" wire #0 is not a wire but a mode of operation
        // uses the special input switches on the operator console
        let wire_spool = sub_order & 0b11;

        if wire_spool == 0 && operation == 0o3 {
            operation = 0o2;
        }

        if backward && operation == 0o3 || wire_spool == 0 && operation == 0o0 {
            self.halt(next_addr);
            return;
        }

        // FuncDesc Diagram 104-4LC-3 "Wire Order Selector"
        let mut mem_index = start;
        loop {
            if backward {
                self.translate_wire(wire_spool, WireShift::Backward(BIT_WIDTH));
            }

            match operation {
                0o0 => {
                    // Translate, do nothing
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
                    // Read 5th Addr (a.k.a. R5A)
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
                self.state.initial_address_register = addresses[3];

                return;
            } else {
                // this does nothing if operation == read fifth address, because
                // mem_index gets reset later
                mem_index = (mem_index + 1) & ADDRESS_MASK as usize;
            }
        }
    }

    fn execute_sub(&mut self, addresses: [usize; 4]) {
        let a = self.get(addresses[0]);
        let b = self.get(addresses[1]);
        let (difference, did_overflow) = a.overflowing_sub(b);

        self.set(addresses[2], difference);

        if did_overflow {
            self.handle_overflow(false, addresses[3]);
        } else {
            self.state.initial_address_register = addresses[3];
        }
    }

    fn execute_extract(&mut self, addresses: [usize; 4]) {
        let mut a = self.get(addresses[0]).get_bits();
        let stored_sign = a & 0b1;
        a &= !0b1;

        let dest = addresses[2];
        let mut result = self.get(dest).get_bits();

        let shift_code = addresses[1];
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

        self.state.initial_address_register = addresses[3];
    }

    fn execute_mul(&mut self, addresses: [usize; 4], exact: bool) {
        let a = self.get(addresses[0]);
        let b = self.get(addresses[1]);
        let (rounded, extra_precision) = a.mul(b);

        let dest = addresses[2];
        self.set(dest, rounded);

        if exact {
            self.set((dest + 1) & ADDRESS_MASK as usize, extra_precision);
        }

        self.state.initial_address_register = addresses[3];
    }

    fn execute_div(&mut self, addresses: [usize; 4], exact: bool) {
        let a = self.get(addresses[0]);
        let b = self.get(addresses[1]);
        let (rounded, extra_precision, overflow) = a.overflowing_div(b);

        let dest = addresses[2];
        self.set(dest, rounded);

        if exact {
            self.set((dest + 1) & ADDRESS_MASK as usize, extra_precision);
        }

        // to-do: Note about rounded division on FuncDesc 4-31

        if overflow {
            self.handle_overflow(true, addresses[3]);
        } else {
            self.state.initial_address_register = addresses[3];
        }
    }

    // This is for executing the order `Halt`; `Edvac::halt` is for whenever the
    // machine needs to stop.
    fn execute_halt(&mut self, addresses: [usize; 4]) {
        self.halt(addresses[3]);
    }

    /// Decodes and executes the *provided* order, returning the next order that
    /// is along the execution path, or None.
    pub fn execute_once(&mut self, order: &Order) {
        let addresses = order.addresses;

        match order.kind {
            OrderKind::Compare => self.execute_compare(addresses),
            OrderKind::ManualRead => self.execute_manual_read(addresses),
            OrderKind::Add => self.execute_add(addresses),
            OrderKind::Wire => self.execute_wire(addresses),
            OrderKind::Sub => self.execute_sub(addresses),
            OrderKind::Extract => self.execute_extract(addresses),
            OrderKind::Mul => self.execute_mul(addresses, false),
            OrderKind::MulExact => self.execute_mul(addresses, true),
            OrderKind::Div => self.execute_div(addresses, false),
            OrderKind::DivExact => self.execute_div(addresses, true),
            OrderKind::Halt => self.execute_halt(addresses),

            OrderKind::Unused => self.halt(addresses[3]),
        }
    }
}
