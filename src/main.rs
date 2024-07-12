mod assembler;
mod system;

use std::{fs, path::PathBuf};

use assembler::Assembler;
use clap::Parser;

pub const OPCODE: u16 = 0xF000;
pub const ADDRESS: u16 = 0x0FFF;
pub const CONDITION: u16 = 0x0C00;

pub const JNS: u16 = 0x0000;
pub const LOAD: u16 = 0x1000;
pub const STORE: u16 = 0x2000;
pub const ADD: u16 = 0x3000;
pub const SUBT: u16 = 0x4000;
pub const INPUT: u16 = 0x5000;
pub const OUTPUT: u16 = 0x6000;
pub const HALT: u16 = 0x7000;
pub const SKIPCOND: u16 = 0x8000;
pub const JUMP: u16 = 0x9000;
pub const CLEAR: u16 = 0xA000;
pub const ADDI: u16 = 0xB000;
pub const JUMPI: u16 = 0xC000;
pub const LOADI: u16 = 0xD000;
pub const STOREI: u16 = 0xE000;

pub const LESS: u16 = 0b00 << 10;
pub const EQUAL: u16 = 0b01 << 10;
pub const GREATER: u16 = 0b10 << 10;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(args.file).unwrap();

    let mut assembler = Assembler::new();
    assembler.process(&file);

    let mut system = assembler.generate_system();
    system.run();

    if args.debug {
        println!("{system}");
    }
}
