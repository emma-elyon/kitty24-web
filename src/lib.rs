use virtual_machine::VirtualMachine;

/// Create new virtual machine.
#[no_mangle]
fn virtual_machine() -> *mut VirtualMachine {
    Box::into_raw(Box::new(VirtualMachine::default()))
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

/// Run virtual machine for one frame.
#[no_mangle]
fn run(virtual_machine: &mut VirtualMachine) {
    virtual_machine.run();
}
