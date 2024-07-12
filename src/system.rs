use std::{
    fmt::Display,
    io,
    ops::{Index, IndexMut},
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::{
    ADD, ADDI, ADDRESS, CLEAR, CONDITION, EQUAL, GREATER, HALT, INPUT, JNS, JUMP, JUMPI, LESS,
    LOAD, LOADI, OPCODE, OUTPUT, SKIPCOND, STORE, STOREI, SUBT,
};

pub struct System {
    pc: u16,
    ir: u16,
    ac: u16,
    mar: u16,
    mbr: u16,
    inreg: u16,
    outreg: u16,
    m: Memory,
}

impl System {
    pub fn new(start_address: u16, instructions: &[u16]) -> Self {
        let mut memory = [0; 0x1000];
        for (instruction, address) in instructions.iter().copied().zip(start_address as usize..) {
            memory[address] = instruction;
        }
        Self {
            pc: start_address,
            ir: 0,
            ac: 0,
            mar: 0,
            mbr: 0,
            inreg: 0,
            outreg: 0,
            m: memory.into(),
        }
    }

    pub fn run(&mut self) {
        let stdin_channel = spawn_stdin_channel();

        loop {
            if let Ok(input) = stdin_channel.try_recv() {
                self.inreg = input
            }

            // fetch
            self.mar = self.pc;
            self.ir = self.m[self.mar];
            self.pc += 1;

            // decode
            let opcode = self.ir & OPCODE;
            self.mar = self.ir & ADDRESS;
            self.mbr = self.m[self.mar];

            if opcode == 0 {
                break;
            }

            // execute
            match opcode {
                JNS => {
                    self.mbr = self.pc;
                    self.m[self.mar] = self.mbr;
                    self.mbr = self.mar;
                    self.ac = 1 + self.mbr;
                    self.pc = self.ac;
                }
                LOAD => self.ac = self.mbr,
                STORE => self.m[self.mar] = self.ac,
                ADD => self.ac += self.mbr,
                SUBT => self.ac -= self.mbr,
                INPUT => self.ac = self.inreg,
                OUTPUT => {
                    self.outreg = self.ac;
                    println!("{}", self.outreg);
                }
                HALT => break,
                SKIPCOND => {
                    let skip = match self.mar & CONDITION {
                        LESS => (self.ac as i16) < 0,
                        EQUAL => (self.ac as i16) == 0,
                        GREATER => (self.ac as i16) > 0,
                        _ => panic!(),
                    };
                    if skip {
                        self.pc += 1;
                    }
                }
                JUMP => self.pc = self.mar,
                CLEAR => self.ac = 0,
                ADDI => self.ac += self.m[self.mbr],
                JUMPI => self.pc = self.mbr,
                LOADI => self.ac = self.m[self.mbr],
                STOREI => self.m[self.mbr] = self.ac,
                _ => panic!(),
            }
        }
    }
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            pc,
            ir,
            ac,
            mar,
            mbr,
            inreg,
            outreg,
            ..
        } = self;
        write!(
            f,
            "Program Counter: 0x{pc:0X} {pc}
Instruction Register: 0x{ir:0X} {ir}
Accumulator: 0x{ac:0X} {ac}
Memory Access Register: 0x{mar:0X} {mar}
Memory Buffer Register: 0x{mbr:0X} {mbr}
Input Register: 0x{inreg:0X} {inreg}
Output Register: 0x{outreg:0X} {outreg}",
        )
    }
}

fn spawn_stdin_channel() -> Receiver<u16> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        sender.send(buffer.trim().parse().unwrap()).unwrap();
    });
    receiver
}

struct Memory([u16; 0x1000]);

impl From<[u16; 0x1000]> for Memory {
    fn from(value: [u16; 0x1000]) -> Self {
        Self(value)
    }
}

impl Index<u16> for Memory {
    type Output = u16;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
