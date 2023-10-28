mod vblank {
    use common::INTERRUPT_VBLANK;

    use crate::common::run_virtual_machine_without_harness;

    #[test]
    fn vblank_happens_in_the_first_frame() {
        let [sp, ..] = run_virtual_machine_without_harness(
            r"
            lessi   rF, ir, 0
            caddi   pc, pc, ~main
            ori     sp, ir, 0
            let     ir, 0
            main:
                subi    pc, pc, ~main
        ",
        );
        assert_eq!(sp, INTERRUPT_VBLANK);
    }
}

mod nested {
    use crate::common::run_virtual_machine_without_harness;

    #[test]
    fn nested_interrupts_restore_registers_and_condition() {
        let [_, r1, r2, ..] = run_virtual_machine_without_harness(
            r"
            lessi   rF, ir, 0
            caddi   pc, pc, ~main
            lessi   rF, ir, 2
            let     rA, 0xFF0000
            lethi   rA, 0xFF0000
            clet    r1, 17
            clet    ir, 1
            cstore  rA, r1, 0
            clet    ir, 0
            let     r2, 34
            store   rA, r2, 1
            let     ir, 0
            main:
                let     rA, 0xFF0000
                let     ir, 2
                lethi   rA, 0xFF0000
            loop:
                load    r1, rA, 0
                load    r2, rA, 1
                subi    pc, pc, ~loop
        ",
        );
        assert_eq!(r1, 17);
        assert_eq!(r2, 34);
    }
}