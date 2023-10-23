use assembler::Assembler;
use virtual_machine::*;

pub fn run_virtual_machine(source: &str) -> [u32; REGISTER_COUNT] {
    match Assembler::assemble(source) {
        Ok(rom) => {
            let mut vm = VirtualMachine::new(rom);
            vm.run();
            vm.registers()
        },
        Err(error) => panic!("{}", error)
    }
}
