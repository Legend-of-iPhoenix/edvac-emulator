#[macro_export]
macro_rules! inst {
    (@args $a1:literal $a2:literal $a3:literal $a4:literal) => {
        (($a1 as u64) << 34) | (($a2 as u64) << 24) | (($a3 as u64) << 14) | (($a4 as u64) << 4)
    };

    (C $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b0010)
    };

    (MR $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b0011)
    };

    (A $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b0100)
    };

    (W $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b0101)
    };

    (S $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b0110)
    };

    (E $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b0111)
    };

    (M $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b1000)
    };

    (m $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b1001)
    };

    (D $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b1010)
    };

    (d $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b1011)
    };

    (H $a1:literal $a2:literal $a3:literal $a4:literal) => {
        crate::word::Word::with_bits(inst!(@args $a1 $a2 $a3 $a4) | 0b1100)
    };
}
