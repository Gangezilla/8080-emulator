use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

mod bit;
mod memory;

extern crate assert_cli;

struct Flag {
    z: bool,  // Zero Flag
    s: bool,  // Sign Flag
    p: bool,  // Parity Flag
    cy: bool, // Carry Flag
    ac: bool, // Auxilary Carry Flag
    pad: bool,
}

impl Flag {
    pub fn new() -> Flag {
        Flag {
                z: false,
                s: false,
                p: false,
                cy: false,
                ac: false,
                pad: false,
            }
    }

    // pub fn set_flag(&mut self, flag: Flag, b: bool) {
    //     if b {
    //         self.flag = bit::set_bit(self.flag, flag as usize)
    //     } else {
    //         self.flag = bit::clear_bit(self.flag, flag as usize)
    //     }
    // }

    pub fn set_z(&mut self, value: bool) {
        self.z = value;
    }
}

pub struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Rc<RefCell<memory::Memory>>,
    int_enable: u8,
}

impl State8080 {
    pub fn new(mem: Rc<RefCell<memory::Memory>>) -> State8080 {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x00,
            memory: mem,
            int_enable: 0,
        }
    }

    pub fn get_current_pc(&mut self) -> u8 {
        // called imm_ds???
        let opcode = self.memory.borrow().get(self.pc);
        self.pc += 1;
        return opcode;
    }

    fn alu_add(&mut self, flag: &Flag, byte: u8) {
        let a = self.a;
        let result = a.wrapping_add(byte);
        &flag.set_z(bit::get_bit(result, 7));
        // flag.set_flag('z', bit::get_bit(result, 7)); // 7 because we're getting the most significant bit

    }
}


fn unimplemented_instruction(instruction: u8) {
    eprintln!("Error: Unimplemented instruction ---- {:x}", instruction);
    std::process::exit(1);
}

fn emulate_8080(state: &State8080, flag: &Flag, opcode: u8) {
    match opcode {
        0x00 => (),
        // Arithmetic
        0x80 => {
            // ADD B
            state.alu_add(&flag, state.b)
        }
        // Data Transfer
        // Logical
        // Branch
        // Stack
        // I/O
        // Special
        _ => unimplemented_instruction(opcode),
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let mem = Rc::new(RefCell::new(memory::Memory::new()));
    let mut file = File::open(&filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    mem.borrow_mut().data[0x00..buf.len()].clone_from_slice(&buf[..]);

    let mut state = State8080::new(mem.clone());
    let mut flag = Flag::new();

    loop {
        let mut cycle = 0;
        // implement cycles
        // implement interrupts
        // implement drawing pixels
        // implement handling joypad


        while cycle < 10 {
            // cycle += emulate_8080(&state);
            let opcode = state.get_current_pc();
            emulate_8080(&state, &flag, opcode);
            cycle += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn calling_emulator_without_filename() {
        assert_cli::Assert::main_binary().fails().unwrap();
    }
}