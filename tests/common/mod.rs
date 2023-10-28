use assembler::Assembler;
use common::REGISTER_COUNT;
use virtual_machine::*;

pub fn run_virtual_machine(source: &str) -> [u32; REGISTER_COUNT] {
    let source = r"
        lessi   rF, ir, 0
        caddi   pc, pc, ~__main
        let     ir, 0
        __main:
    ".to_string() + source + r"
        __loop:
            subi    pc, pc, ~__loop
    ";
    match Assembler::assemble(source.as_str()) {
        Ok(rom) => {
            let mut vm = VirtualMachine::new(rom);
            vm.run();
            vm.registers()
        }
        Err(error) => panic!("{}", error),
    }
}

pub fn run_virtual_machine_without_harness(source: &str) -> [u32; REGISTER_COUNT] {
    match Assembler::assemble(source) {
        Ok(rom) => {
            let mut vm = VirtualMachine::new(rom);
            vm.run();
            vm.registers()
        }
        Err(error) => panic!("{}", error),
    }
}