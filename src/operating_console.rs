//
// todo: bin-octal switches
pub struct BinarySwitchArray {
    value: u64,
    bit_len: usize,
}

impl BinarySwitchArray {
    pub fn switch_set(&mut self, index: usize, switch_value: bool) {
        assert!(index < self.bit_len);

        self.value = if switch_value {
            self.value | 0b1 << index
        } else {
            self.value & !(0b1 << index)
        };
    }

    #[must_use]
    pub fn read(&self) -> u64 {
        self.value
    }

    #[must_use]
    pub fn new(bit_len: usize) -> BinarySwitchArray {
        assert!(bit_len < 64);

        BinarySwitchArray { value: 0, bit_len }
    }
}

// Origins+Fate pg. 34
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemoryMode {
    L0,
    LR,
    R1,
}

impl Default for MemoryMode {
    fn default() -> Self {
        MemoryMode::LR
    }
}

// Conflicting information is available on the excess capacity actions. FuncDesc
// includes an aside in section 1.5 that the last two excess capacity actions do
// *not* "determine the position of the next order to be executed". Origins+Fate
// makes no mention of this, and even goes on to describe the last position as a
// "Jump", which implies (at least, in my opinion) that they *do*. I think it is
// somewhat more likely that the author of FuncDesc was blind to this aside than
// this particular implementation detail changing in the middle of construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExcessCapacityAction {
    Halt,
    Ignore,
    ExecuteSpecial,
    ExecuteAddressB,
}

impl Default for ExcessCapacityAction {
    fn default() -> Self {
        ExcessCapacityAction::Halt
    }
}

pub struct State {
    pub initial_address_register: usize,
    pub excess_capacity_action: ExcessCapacityAction,
    pub memory_mode: MemoryMode,

    pub auxiliary_input_switches: BinarySwitchArray,

    pub special_order_switches: BinarySwitchArray,
    pub address_a_switches: BinarySwitchArray,
    pub address_b_switches: BinarySwitchArray,
}

impl Default for State {
    fn default() -> Self {
        State {
            initial_address_register: 0,
            excess_capacity_action: Default::default(),
            memory_mode: Default::default(),

            auxiliary_input_switches: BinarySwitchArray::new(44),

            special_order_switches: BinarySwitchArray::new(44),
            address_a_switches: BinarySwitchArray::new(10),
            address_b_switches: BinarySwitchArray::new(10),
        }
    }
}
