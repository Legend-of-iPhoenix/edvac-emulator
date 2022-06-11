use crate::word::Word;

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

    pub auxiliary_input_switches: Word,

    pub special_order_switches: Word,
    pub address_a_switches: usize,
    pub address_b_switches: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            initial_address_register: 0,
            excess_capacity_action: Default::default(),
            memory_mode: Default::default(),

            auxiliary_input_switches: 0_i64.try_into().unwrap(),

            special_order_switches: 0_i64.try_into().unwrap(),
            address_a_switches: 0,
            address_b_switches: 0,
        }
    }
}
