#[macro_export]
macro_rules! set1 {
    ($x: expr, $offset: literal, u16) => {
        $x |= 1 << (15 - $offset);
    };
}

#[macro_export]
macro_rules! set0 {
    ($x: expr, $offset: literal, u16) => {
        $x &= !(1 << (15 - $offset));
    };
}

#[macro_export]
macro_rules! get_bits {
    ($x: expr, $offset: literal, $length: literal, u16) => {
        // for example: $x = 1011010011110101
        //              $offset = 5
        //              $length = 7
        //
        // assert: 0 < $offset + $length <= 16
        // expected: 1001111
        //
        // step-1: $x >> 4 (16 - $offset - $length)
        //         $x = 101101001111
        // step-2: $x & 000001111111 ((1 << $length) - 1)
        //         $x = 1001111
        ($x >> (16 - $offset - $length)) & ((1 << $length) - 1)
    };
}

#[macro_export]
macro_rules! get_bit {
    ($x: expr, $offset: literal, u16) => {
        get_bits!($x, $offset, 1, u16)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_bits() {
        assert_eq!(get_bits!(0b1011010011110101, 5, 7, u16), 0b1001111);
    }

    #[test]
    fn test_get_bit() {
        assert_eq!(get_bit!(0b1011010011110101, 0, u16), 1);
        assert_eq!(get_bit!(0b1011010011110101, 1, u16), 0);
        assert_eq!(get_bit!(0b1011010011110101, 7, u16), 0);
        assert_eq!(get_bit!(0b1011010011110101, 8, u16), 1);
    }
}
