use assembler::Assembler;
use virtual_machine::VirtualMachine;

fn main() {
    let assembly = include_str!("boot.kittyasm");
    match Assembler::assemble(assembly) {
        Ok(rom) => {
            let mut virtual_machine = VirtualMachine::new(rom);
            for _ in 0..60 {
                virtual_machine.run();
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
