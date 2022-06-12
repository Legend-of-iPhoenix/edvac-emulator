//! This program comes from <https://doi.org/10.2307/2002881>

use crate::{word::Word, Edvac};

fn load(computer: &mut Edvac) {
    computer.high_speed_memory.load(vec![
        (0o0000, inst!(E 0o1001 0o0003 0o1003 0o1003)),
        (0o1003, inst!(W 0o0001 0o0202 0o0000 0o1002)),
        (0o1002, inst!(E 0o1003 0o0241 0o1006 0o1007)),
        (0o1007, inst!(E 0o1003 0o0003 0o1055 0o1006)),
        (0o1006, inst!(E 0o0000 0o0007 0o1054 0o1005)),
        (0o1005, inst!(E 0o1006 0o1243 0o1050 0o1011)),
        (0o1011, inst!(E 0o1054 0o0447 0o1047 0o1004)),
        (0o1004, inst!(M 0o1016 0o1047 0o1052 0o1013)),
        (0o1013, inst!(E 0o1054 0o0407 0o1056 0o1010)),
        (0o1010, inst!(A 0o1052 0o1056 0o1047 0o1012)),
        (0o1012, inst!(M 0o1016 0o1047 0o1052 0o1023)),
        // page turn!
        (0o1023, inst!(E 0o1054 0o0347 0o1056 0o1017)),
        (0o1017, inst!(A 0o1052 0o1056 0o1047 0o1014)),
        (0o1014, inst!(M 0o1016 0o1047 0o1052 0o1033)),
        (0o1033, inst!(E 0o1054 0o0307 0o1056 0o1020)),
        (0o1020, inst!(A 0o1052 0o1056 0o1047 0o1015)),
        (0o1015, inst!(M 0o1016 0o1047 0o1052 0o1043)),
        (0o1043, inst!(E 0o1054 0o0247 0o1056 0o1021)),
        (0o1021, inst!(A 0o1052 0o1056 0o1047 0o1022)),
        (0o1022, inst!(M 0o1016 0o1047 0o1052 0o1053)),
        (0o1053, inst!(E 0o1054 0o0207 0o1056 0o1027)),
        (0o1027, inst!(A 0o1052 0o1056 0o1047 0o1024)),
        (0o1024, inst!(M 0o1016 0o1047 0o1052 0o1063)),
        (0o1063, inst!(E 0o1054 0o0147 0o1056 0o1030)),
        (0o1030, inst!(A 0o1052 0o1056 0o1047 0o1025)),
        (0o1025, inst!(M 0o1016 0o1047 0o1052 0o1073)),
        (0o1073, inst!(E 0o1054 0o0107 0o1056 0o1031)),
        (0o1031, inst!(A 0o1052 0o1056 0o1047 0o1032)),
        (0o1032, inst!(M 0o1016 0o1047 0o1052 0o1103)),
        (0o1103, inst!(E 0o1054 0o0047 0o1056 0o1041)),
        (0o1041, inst!(A 0o1052 0o1056 0o1047 0o1051)),
        (0o1051, inst!(M 0o1026 0o1047 0o1052 0o1113)),
        (0o1113, inst!(A 0o1054 0o1052 0o1044 0o1050)),
        (0o1050, inst!(d 0o1044 0o1035 0o0000 0o1045)),
        (0o1045, inst!(S 0o1006 0o1042 0o1006 0o1037)),
        (0o1037, inst!(C 0o1006 0o1042 0o1055 0o1006)),
        (0o1055, inst!(W 0o0001 0o0103 0o0000 0o1040)),
        (0o1040, inst!(C 0o1001 0o1042 0o1034 0o1036)),
        (0o1036, inst!(S 0o1001 0o1042 0o1001 0o1046)),
        (0o1046, inst!(E 0o1042 0o1133 0o1003 0o1003)),
        (0o1034, inst!(H 0o0000 0o0000 0o0000 0o0000)),
        // constants
        (0o1016, Word::from_bits(0o014_6314_6314_6315 << 1)), // +1/10
        (0o1026, Word::from_bits(0o004_6314_6314_6315 << 1)), // +3/80
        (0o1035, Word::from_bits(0o120_0000_0000_0000 << 1)), // +5/8
        (0o1042, Word::from_bits(0o000_1000_0000_0000 << 1)), // +2^-10
        (0o1001, (1 << 13).try_into().unwrap()),
    ]);
}

#[test]
fn works() {
    let mut computer = Edvac::default();

    load(&mut computer);

    computer.low_speed_memory[1].write_word(Word::from_bits(
        0b0010_0010_0010_0010_0010_0010_0010_0010_0010_0010_0000,
    ));

    computer.initiate();
    computer.continue_to_completion();

    assert_eq!(
        computer.high_speed_memory.dump()[0o0001],
        Word::from_bits(0b00111000111000111000111000111000110010110000)
    );
}
