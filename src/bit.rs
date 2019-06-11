pub fn get_bit(number: u8, byte: usize) -> bool {
    (number & (1 << byte)) != 0
    // shifts 1 n bytes to the left.
    // & returns a 1 in each bit position when the corresponding bits are 1
}

pub fn set_bit(number: u8, byte: usize) -> u8 {
    number | (1 << byte)
    // shifts 1 n bytes to the left.
    // | returns a 1 in each bit position when either corresponding bit is 1
}

pub fn clear_bit(number: u8, byte: usize) -> u8 {
    number & !(1 << byte)
}