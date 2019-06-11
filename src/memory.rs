
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn get(&self, a: u16) -> u8 {
        self.data[usize::from(a)]
    }

    pub fn set(&mut self, a: u16, v: u8) {
        self.data[usize::from(a)] = v
    }

    pub fn new() -> Self {
        Self {
            data: vec![0; 65536],
        }
    }
}