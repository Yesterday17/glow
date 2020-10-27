use std::convert::TryInto;

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

#[macro_export]
macro_rules! u8_merge {
    ($x: expr, $y: expr) => {
        (($x as u16) << 8) | $y as u16
    };
}

pub trait EasyMerge<K, V> {
    fn merge_lower(&self, lower: K) -> V;
    fn merge_higher(&self, higher: K) -> V;
}

impl EasyMerge<u8, u16> for u8 {
    fn merge_lower(&self, lower: u8) -> u16 {
        u8_merge!(*self, lower)
    }

    fn merge_higher(&self, higher: u8) -> u16 {
        u8_merge!(higher, *self)
    }
}

macro_rules! parse_num {
    ($from: ident, $to: ident, $len: expr) => {
        let to_array =
            |slice: &[u8]| -> [u8; $len] { slice.try_into().expect("slice with incorrect length") };

        return $to::from_le_bytes(to_array($from));
    };
}

pub fn parse_u32(from: &[u8], network_order: bool) -> u32 {
    parse_num!(from, u32, 4);
}

pub fn parse_some_u32(from: &[u8], network_order: bool) -> Option<u32> {
    Some(parse_u32(from, network_order))
}

#[cfg(test)]
mod tests {
    const TEST_U16: u16 = 0b1011010011110101;

    #[test]
    fn test_get_bits() {
        assert_eq!(get_bits!(TEST_U16, 5, 7, u16), 0b1001111);
    }

    #[test]
    fn test_get_bit() {
        assert_eq!(get_bit!(TEST_U16, 0, u16), 1);
        assert_eq!(get_bit!(TEST_U16, 1, u16), 0);
        assert_eq!(get_bit!(TEST_U16, 7, u16), 0);
        assert_eq!(get_bit!(TEST_U16, 8, u16), 1);
    }

    #[test]
    fn merge_u8() {
        use super::EasyMerge;
        let higher = get_bits!(TEST_U16, 0, 8, u16) as u8;
        let lower = get_bits!(TEST_U16, 8, 8, u16) as u8;
        assert_eq!(TEST_U16, higher.merge_lower(lower));
        assert_eq!(TEST_U16, lower.merge_higher(higher));
    }
}
