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

#[cfg(test)]
mod tests {
  #[test]
  fn get_bit_shift_1_by_1_byte() {
    assert_eq!(super::get_bit(1, 1), false);
  }

  #[test]
  fn get_bit_shift_1_by_0_bytes() {
    assert_eq!(super::get_bit(1, 0), true);
  }

  #[test]
  fn get_bit_shift_0_by_1_byte() {
    assert_eq!(super::get_bit(0, 1), false);
  }

  #[test]
  fn get_bit_shift_0_by_0_bytes() {
    assert_eq!(super::get_bit(0, 0), false);
  }
}

// The pointer-sized unsigned integer type.
// The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
