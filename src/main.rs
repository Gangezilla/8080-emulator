use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

struct ConditionCodes {
    z: u8,  // Zerp F;ag
    s: u8,  // Sign Flag
    p: u8,  // Parity Flag
    cy: u8, // Carry Flag
    ac: u8, // Auxilary Carry Flag
    pad: u8,
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
    memory: Rc<RefCell<Memory>>,
    condition_codes: ConditionCodes,
    int_enable: u8,
}

impl State8080 {
    pub fn new(mem: Rc<RefCell<Memory>>) -> State8080 {
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
            condition_codes: ConditionCodes {
                z: 0,
                s: 0,
                p: 0,
                cy: 0,
                ac: 0,
                pad: 0,
            },
            int_enable: 0,
        }
    }

    pub fn set_b(&mut self, v: u8) {
        self.b = v;
    }

    pub fn set_c(&mut self, v: u8) {
        self.c = v;
    }

    pub fn get_current_pc(&mut self) -> u8 {
        // called imm_ds???
        let opcode = self.memory.borrow().get(self.pc);
        self.pc += 1;
        return opcode;
    }
}

pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    fn get(&self, a: u16) -> u8 {
        self.data[usize::from(a)]
    }

    fn set(&mut self, a: u16, v: u8) {
        self.data[usize::from(a)] = v
    }

    pub fn new() -> Self {
        Self {
            data: vec![0; 65536],
        }
    }
}


fn unimplemented_instruction(instruction: u8) {
    eprintln!("Error: Unimplemented instruction ---- {:x}", instruction);
    std::process::exit(1);
}

// fn emulate_8080(state: &State8080) { // needs to return u32 later
//     let opcode = state.get_current_pc();
//     println!("{}", opcode);
//     // match &hex {
//     //     0x00 => (),
//     //     0x01 => {
//     //         state.set_c();
//     //         state.set_b();
//     //         state.increment_pc(2);
//     //     }
//     //     _ => (),
//     // }
// }

fn emulate_8080(state: &State8080, opcode: u8) {
    match opcode {
        0x00 => (),
        // Arithmetic
        0x80 => {
            // ADD B

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
    let mem = Rc::new(RefCell::new(Memory::new()));
    let mut file = File::open(&filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    mem.borrow_mut().data[0x00..buf.len()].clone_from_slice(&buf[..]);

    let mut state = State8080::new(mem.clone());

    loop {
        let mut cycle = 0;
        // implement cycles
        // implement interrupts
        // implement drawing pixels
        // implement handling joypad


        while cycle < 10 {
            // cycle += emulate_8080(&state);
            let opcode = state.get_current_pc();
            emulate_8080(&state, opcode);
            cycle += 1;
        }
    }
}