use crate::word::Word;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OperatingMode {
    // SpecialOneCycle,
    // SpecialOneExecute,
    SpecialOneOrder,

    NormalToCompletion,
    NormalToAddressA,
    // NormalOneCycle,
    // NormalOneExecute,
    NormalOneOrder,
}

impl Default for OperatingMode {
    fn default() -> Self {
        Self::NormalToCompletion
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
// somewhat more probable that the author of Origins+Fate ignored this note than
// this particular implementation detail changing in the middle of construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExcessCapacityAction {
    Halt,
    Ignore,
    ExecuteSpecial,
    ExecuteAddressB,
}

// Per Prelim. Report pg 83, the default is "Halt" because overflows usually are
// signs of errors in programming.
impl Default for ExcessCapacityAction {
    fn default() -> Self {
        ExcessCapacityAction::Halt
    }
}

pub struct State {
    pub initial_address_register: usize,
    pub operating_mode: OperatingMode,

    pub excess_capacity_action_add: ExcessCapacityAction,
    pub excess_capacity_action_div: ExcessCapacityAction,
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
            operating_mode: Default::default(),

            excess_capacity_action_add: Default::default(),
            excess_capacity_action_div: Default::default(),

            memory_mode: Default::default(),

            auxiliary_input_switches: 0_i64.try_into().unwrap(),

            special_order_switches: 0_i64.try_into().unwrap(),
            address_a_switches: 0,
            address_b_switches: 0,
        }
    }
}
