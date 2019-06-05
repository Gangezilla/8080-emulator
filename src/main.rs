use std::env;
use std::fs::File;
use std::io::Read;
use std::cell::RefCell;


struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
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
    pc: std::vec::Vec<std::cell::RefCell<u16>>, // this is very likely wrong...
    memory: *mut u8,
    condition_codes: ConditionCodes,
    int_enable: u8,
}

impl State8080 {
    pub fn new(mem: Vec<RefCell<u16>>) -> State8080 {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: mem,
            memory: std::ptr::null_mut(),
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
}

// fn unimplemented_instruction(instruction: &str) {
//     eprintln!("Error: Unimplemented instruction ---- {:x}", instruction);
//     std::process::exit(1);
// }

fn emulate_8080(state: &State8080, hex: u8) {
    match &hex {
        0x00 => (),
        0x01 => {
            state.set_c();
            state.set_b();

        }
        _ => (),
    }
}

// gets the opcode from state
// how do i load our opcode into state. 
//first hurdle yeah, really is getting my opcodes into state...

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mem = RefCell::new(&buffer[..]);

    let mut state = State8080::new(mem);
    // let mut position = 0;
    // let end_position = buffer.len();
    // while position < end_position {
    //     let hex = &format!("{:x}", buffer[position]);
    //     emulate_8080(&state, hex)
    // }
    // unimplemented_instruction();
}