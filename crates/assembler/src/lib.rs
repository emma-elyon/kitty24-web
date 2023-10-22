use std::collections::HashMap;

use common::Op;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "kittyasm.pest"]
struct KittyAssemblyParser;

struct LabelReference {
    identifier: String,
    address: u32,
    length: u32,
    shift: u32,
}

pub struct Assembler {
    bytes: Vec<u8>,
    labels: HashMap<String, u32>,
    scope: String,
    relative_references: Vec<LabelReference>,
    absolute_references: Vec<LabelReference>,
}

impl Assembler {
    pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
        match KittyAssemblyParser::parse(Rule::Program, source) {
            // The parse was successful; unwrap cannot fail here.
            Ok(mut program) => Self::default().parse_program(program.next().unwrap()),
            Err(error) => Err(error.to_string()),
        }
    }

    fn parse_program(&mut self, pair: Pair<Rule>) -> Result<Vec<u8>, String> {
        for statement in pair.into_inner() {
            match statement.as_rule() {
                Rule::Instruction => self.parse_instruction(statement),
                Rule::LabelDefinition => self.parse_label_definition(statement),
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }
        for LabelReference {
            identifier,
            address,
            length,
            shift,
        } in &self.absolute_references
        {
            if let Some(&target) = self.labels.get(identifier) {
                let mask = 2_u32.pow(*length) - 1;
                let u = target;
                let u = u >> shift;
                let u = u & mask;
                let [a, b, c]: [u8; 3] = self.bytes[*address as usize..*address as usize + 2]
                    .try_into()
                    .unwrap();
                let instruction = u32::from_be_bytes([0, a, b, c]);
                let instruction = instruction | u;
                let [_, a, b, c] = instruction.to_be_bytes();
                self.bytes[*address as usize + 0] = a;
                self.bytes[*address as usize + 1] = b;
                self.bytes[*address as usize + 2] = c;
            } else {
                panic!("Unknown label: {}", identifier);
            }
        }
        for LabelReference {
            identifier,
            address,
            length,
            shift,
        } in &self.relative_references
        {
            if let Some(&target) = self.labels.get(identifier) {
                let mask = 2_u32.pow(*length) - 1;
                // TODO: Error on negative addi or positive subi
                // TODO: Maybe pseudo-instructions `jump`/`cjump` and maybe `letall`/`cletall`
                let u = (target as i32 - (*address as i32 + 3)).abs() as u32;
                let u = u >> shift;
                let u = u & mask;
                let [a, b, c]: [u8; 3] = self.bytes[*address as usize..*address as usize + 3]
                    .try_into()
                    .unwrap();
                let instruction = u32::from_be_bytes([0, a, b, c]);
                let instruction = instruction | u;
                let [_, a, b, c] = instruction.to_be_bytes();
                self.bytes[*address as usize + 0] = a;
                self.bytes[*address as usize + 1] = b;
                self.bytes[*address as usize + 2] = c;
            } else {
                panic!("Unknown label: {}", identifier);
            }
        }
        Ok(self.bytes.clone())
    }

    fn parse_label_definition(&mut self, pair: Pair<Rule>) {
        let label = pair.into_inner().next().unwrap();
        match label.as_rule() {
            Rule::GlobalLabel => self.add_global_label(label),
            Rule::LocalLabel => self.add_local_label(label),
            _ => unreachable!(),
        }
    }

    fn add_global_label(&mut self, pair: Pair<Rule>) {
        let identifier = pair.into_inner().next().unwrap().as_str();
        self.scope = identifier.to_string();
        self.labels
            .insert(identifier.to_string(), self.bytes.len() as u32);
    }

    fn add_local_label(&mut self, pair: Pair<Rule>) {
        let identifier = pair.into_inner().next().unwrap().as_str();
        let identifier = format!("{}.{}", self.scope, identifier.to_string());
        self.labels
            .insert(identifier.to_string(), self.bytes.len() as u32);
    }

    fn parse_instruction(&mut self, pair: Pair<Rule>) {
        let mut pairs = pair.into_inner();
        let op = pairs.next().unwrap();
        match op.as_rule() {
            Rule::OpI => self.parse_immediate(op, pairs),
            Rule::OpL => self.parse_let(op, pairs),
            Rule::OpR => self.parse_register_instruction(op, pairs),
            _ => unreachable!(),
        }
    }

    fn parse_immediate(&mut self, op: Pair<Rule>, mut pairs: Pairs<Rule>) {
        use Op::*;
        let (op, conditional) = match op.as_str().to_lowercase().as_str() {
            "store" => (Store, false),
            "lessi" => (Lessi, false),
            "addi" => (Addi, false),
            "subi" => (Subi, false),
            "cstore" => (Store, true),
            "caddi" => (Addi, true),
            _ => todo!(),
        };
        let conditional = conditional as u32;
        let conditional = conditional << 23;
        let opcode = op as u32;
        let opcode = opcode << 18;
        let r = self.parse_register(pairs.next().unwrap());
        let r = r << 12;
        let s = self.parse_register(pairs.next().unwrap());
        let s = s << 6;
        let u = self.parse_value(pairs.next().unwrap(), 6, 0);
        // TODO: Yell if too big.
        let u = u & 0o77;
        let instruction = conditional | opcode | r | s | u;
        let [_, a, b, c] = instruction.to_be_bytes();
        self.bytes.extend([a, b, c]);
    }

    fn parse_let(&mut self, op: Pair<Rule>, mut pairs: Pairs<Rule>) {
        use Op::*;
        let (op, conditional) = match op.as_str().to_lowercase().as_str() {
            "let" => (Let, false),
            "lethi" => (Lethi, false),
            "clet" => (Let, true),
            "clethi" => (Lethi, true),
            _ => unreachable!(),
        };
        let conditional = conditional as u32;
        let conditional = conditional << 23;
        let opcode = op as u32;
        let opcode = opcode << 18;
        let r = self.parse_register(pairs.next().unwrap());
        let r = r << 12;
        let shift = match op {
            Lethi => 12,
            _ => 0,
        };
        let u = self.parse_value(pairs.next().unwrap(), 12, shift);
        // TODO: Yell if the number is too big to fit?
        let u = match op {
            Let => u & 0o77_77,
            Lethi => (u >> 12) & 0o77_77,
            _ => unreachable!(),
        };
        let instruction = conditional | opcode | r | u;
        let [_, a, b, c] = instruction.to_be_bytes();
        self.bytes.extend([a, b, c]);
    }

    fn parse_register_instruction(&mut self, op: Pair<Rule>, mut pairs: Pairs<Rule>) {
        use Op::*;
        let (op, conditional) = match op.as_str().to_lowercase().as_str() {
            "less" => (Less, false),
            _ => todo!(),
        };
        let conditional = conditional as u32;
        let conditional = conditional << 23;
        let opcode = op as u32;
        let opcode = opcode << 18;
        let r = self.parse_register(pairs.next().unwrap());
        let r = r << 12;
        let s = self.parse_register(pairs.next().unwrap());
        let s = s << 6;
        let t = self.parse_register(pairs.next().unwrap());
        let instruction = conditional | opcode | r | s | t;
        let [_, a, b, c] = instruction.to_be_bytes();
        self.bytes.extend([a, b, c]);
    }

    fn parse_register(&mut self, pair: Pair<Rule>) -> u32 {
        match pair.as_str().to_lowercase().as_str() {
            "r0" => 0x00,
            "r1" => 0x01,
            "r2" => 0x02,
            "r3" => 0x03,
            "r4" => 0x04,
            "r5" => 0x05,
            "r6" => 0x06,
            "r7" => 0x07,
            "r8" => 0x08,
            "r9" => 0x09,
            "ra" => 0x0A,
            "rb" => 0x0B,
            "rc" => 0x0C,
            "rd" => 0x0D,
            "re" => 0x0E,
            "rf" | "pc" => 0x0F,
            "r10" => 0x10,
            "r11" => 0x11,
            "r12" => 0x12,
            "r13" => 0x13,
            "r14" => 0x14,
            "r15" => 0x15,
            "r16" => 0x16,
            "r17" => 0x17,
            "r18" => 0x18,
            "r19" => 0x19,
            "r1a" => 0x1A,
            "r1b" => 0x1B,
            "r1c" => 0x1C,
            "r1d" => 0x1D,
            "r1e" => 0x1E,
            "r1f" => 0x1F,
            "r20" => 0x20,
            "r21" => 0x21,
            "r22" => 0x22,
            "r23" => 0x23,
            "r24" => 0x24,
            "r25" => 0x25,
            "r26" => 0x26,
            "r27" => 0x27,
            "r28" => 0x28,
            "r29" => 0x29,
            "r2a" => 0x2A,
            "r2b" => 0x2B,
            "r2c" => 0x2C,
            "r2d" => 0x2D,
            "r2e" => 0x2E,
            "r2f" => 0x2F,
            "r30" => 0x30,
            "r31" => 0x31,
            "r32" => 0x32,
            "r33" => 0x33,
            "r34" => 0x34,
            "r35" => 0x35,
            "r36" => 0x36,
            "r37" => 0x37,
            "r38" => 0x38,
            "r39" => 0x39,
            "r3a" => 0x3A,
            "r3b" => 0x3B,
            "r3c" => 0x3C,
            "r3d" => 0x3D,
            "r3e" => 0x3E,
            "r3f" => 0x3F,
            register => unreachable!("Register: {}", register),
        }
    }

    fn parse_value(&mut self, pair: Pair<Rule>, length: u32, shift: u32) -> u32 {
        match pair.as_rule() {
            Rule::Number => self.parse_number(pair.into_inner().next().unwrap()),
            Rule::LabelReference => {
                self.parse_label_reference(pair.into_inner().next().unwrap(), length, shift);
                0
            }
            _ => todo!("Value: {}", pair.as_str()),
        }
    }

    fn parse_number(&mut self, pair: Pair<Rule>) -> u32 {
        match pair.as_rule() {
            Rule::Binary => u32::from_str_radix(&pair.as_str()[2..], 0b10).unwrap(),
            Rule::Octal => u32::from_str_radix(&pair.as_str()[1..], 010).unwrap(),
            Rule::Decimal => u32::from_str_radix(pair.as_str(), 10).unwrap(),
            Rule::Hexadecimal => u32::from_str_radix(&pair.as_str()[2..], 0x10).unwrap(),
            _ => unreachable!("Number: {}", pair.as_str()),
        }
    }

    fn parse_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        match pair.as_rule() {
            Rule::RelativeLabelReference => self.parse_relative_label_reference(
                pair.into_inner().next().unwrap(),
                length,
                shift,
            ),
            Rule::AbsoluteLabelReference => self.parse_absolute_label_reference(
                pair.into_inner().next().unwrap(),
                length,
                shift,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_relative_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        match pair.as_rule() {
            Rule::GlobalLabel => self.parse_relative_global_label_reference(
                pair.into_inner().next().unwrap(),
                length,
                shift,
            ),
            Rule::LocalLabel => self.parse_relative_local_label_reference(
                pair.into_inner().next().unwrap(),
                length,
                shift,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_absolute_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        match pair.as_rule() {
            Rule::GlobalLabel => self.parse_absolute_global_label_reference(
                pair.into_inner().next().unwrap(),
                length,
                shift,
            ),
            Rule::LocalLabel => self.parse_absolute_local_label_reference(
                pair.into_inner().next().unwrap(),
                length,
                shift,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_relative_global_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        let identifier = pair.as_str().to_string();
        let address = self.bytes.len() as u32;
        self.relative_references.push(LabelReference {
            identifier,
            address,
            length,
            shift,
        })
    }

    fn parse_absolute_global_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        let identifier = pair.as_str().to_string();
        let address = self.bytes.len() as u32;
        self.absolute_references.push(LabelReference {
            identifier,
            address,
            length,
            shift,
        })
    }

    fn parse_relative_local_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        let identifier = pair.as_str();
        let identifier = format!("{}.{}", self.scope, identifier);
        let address = self.bytes.len() as u32;
        self.relative_references.push(LabelReference {
            identifier,
            address,
            length,
            shift,
        })
    }

    fn parse_absolute_local_label_reference(&mut self, pair: Pair<Rule>, length: u32, shift: u32) {
        let identifier = pair.as_str();
        let identifier = format!("{}.{}", self.scope, identifier);
        let address = self.bytes.len() as u32;
        self.absolute_references.push(LabelReference {
            identifier,
            address,
            length,
            shift,
        })
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            labels: Default::default(),
            scope: Default::default(),
            relative_references: Default::default(),
            absolute_references: Default::default(),
        }
    }
}
