use std::{collections::BTreeMap, str::FromStr};

use crate::{
    system::System, ADD, ADDI, CLEAR, HALT, INPUT, JNS, JUMP, JUMPI, LOAD, LOADI, OUTPUT, SKIPCOND,
    STORE, STOREI, SUBT,
};

enum Symbol {
    Opcode(u16),
    Hex,
    Dec,
    End,
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opcode = match s {
            "JnS" => JNS,
            "Load" => LOAD,
            "Store" => STORE,
            "Add" => ADD,
            "Subt" => SUBT,
            "Input" => INPUT,
            "Output" => OUTPUT,
            "Halt" => HALT,
            "Skipcond" => SKIPCOND,
            "Jump" => JUMP,
            "Clear" => CLEAR,
            "AddI" => ADDI,
            "JumpI" => JUMPI,
            "LoadI" => LOADI,
            "StoreI" => STOREI,
            "Hex" => return Ok(Symbol::Hex),
            "Dec" => return Ok(Symbol::Dec),
            "END" => return Ok(Symbol::End),
            _ => return Err(format!("Could not recognize operation '{s}'")),
        };
        Ok(Symbol::Opcode(opcode))
    }
}

pub struct Assembler {
    start_address: u16,
    instructions: Vec<u16>,
    symbol_table: BTreeMap<String, u16>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            start_address: 0,
            instructions: Vec::new(),
            symbol_table: BTreeMap::new(),
        }
    }

    pub fn process(&mut self, assembly: &str) {
        let mut line_iter = assembly
            .lines()
            .flat_map(|line| line.split('/').next())
            .map(str::trim)
            .filter(|line| !line.is_empty());

        self.start_address = {
            let [org, address] = line_iter
                .next()
                .expect("first noncommented line missing")
                .split_whitespace()
                .collect::<Vec<_>>()[..]
            else {
                panic!("Invalid first line");
            };
            assert_eq!(
                org.to_lowercase(),
                "org",
                "first argument '{org}' is not org"
            );
            u16::from_str_radix(address, 16).expect("Invalid start address")
        };

        let mut lines: Vec<_> = line_iter.collect();

        // first pass
        for (line, address) in lines.iter_mut().zip(self.start_address..) {
            if let Some((literal, instruction)) = line.split_once(",") {
                self.symbol_table.insert(literal.trim().into(), address);
                *line = instruction;
            }
        }

        // second pass
        self.instructions = lines
            .into_iter()
            .flat_map(|instruction| {
                let mut parts = instruction.split_whitespace();
                let symbol = parts.next().unwrap().trim().parse().unwrap();
                let address = parts.next().map(str::trim);

                let value = match symbol {
                    Symbol::Opcode(opcode) if opcode == SKIPCOND => {
                        opcode | u16::from_str_radix(address.unwrap(), 16).unwrap()
                    }
                    Symbol::Opcode(mut instruction) => {
                        if let Some(address) = address {
                            instruction |= address
                                .parse()
                                .unwrap_or_else(|_| self.symbol_table[address]);
                        }
                        instruction
                    }
                    Symbol::Hex => u16::from_str_radix(address.unwrap(), 16).unwrap(),
                    Symbol::Dec => address.unwrap().parse().unwrap(),
                    Symbol::End => return None,
                };
                Some(value)
            })
            .collect();
    }

    pub fn generate_system(&self) -> System {
        System::new(self.start_address, &self.instructions)
    }
}
