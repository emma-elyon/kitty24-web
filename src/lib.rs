use assembler::Assembler;
use virtual_machine::VirtualMachine;

/// Create new virtual machine.
#[no_mangle]
fn virtual_machine() -> *mut VirtualMachine {
    let assembly = include_str!("boot.kittyasm");
    let rom = match Assembler::assemble(assembly) {
        Ok(rom) => rom,
        Err(error) => return Box::into_raw(Box::new(VirtualMachine::error(error))),
    };
    Box::into_raw(Box::new(VirtualMachine::new(rom)))
}

/// Return address of audio output from virtual machine.
#[no_mangle]
fn audio(virtual_machine: &mut VirtualMachine) -> usize {
    virtual_machine.audio.as_ptr() as usize
}

/// Return address of video output from virtual machine.
#[no_mangle]
fn video(virtual_machine: &mut VirtualMachine) -> usize {
    virtual_machine.video.as_ptr() as usize
}

/// Return address of error message from virtual machine.
#[no_mangle]
fn error(virtual_machine: &VirtualMachine) -> usize {
    virtual_machine.error_message.as_ptr() as usize
}

/// Run virtual machine for one frame.
#[no_mangle]
fn run(virtual_machine: &mut VirtualMachine) {
    virtual_machine.run();
}

/// Return length of error message (0 on no error), stored at the beginning
/// of RAM
#[no_mangle]
fn error_message(virtual_machine: &VirtualMachine) -> usize {
    virtual_machine.error_message.len()
}
