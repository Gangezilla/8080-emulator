use std::env;
use std::fs::File;
use std::io::Read;

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
    pc: u16, // where we're up to in mem i think????
    memory: *mut u8,
    condition_codes: ConditionCodes,
    int_enable: u8,
}

impl State8080 {
    pub fn new() -> State8080 {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
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
}

// fn unimplemented_instruction(instruction: &str) {
//     eprintln!("Error: Unimplemented instruction ---- {:x}", instruction);
//     std::process::exit(1);
// }

fn emulate_8080(state: &State8080, hex: &str) {
    match &hex as &str {
        "0" => (),
        "1" => {}
        _ => (),
    }
}


// so this operates on the state, which we have as a struct.
// pass in the struct? or is it like a global thing?

fn main() {

    let filename = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let state = State8080::new();
    let mut position = 0;
    let end_position = buffer.len();
    while position < end_position {
        let hex = &format!("{:x}", buffer[position]);
        emulate_8080(&state, hex)
    }
    // unimplemented_instruction();
}