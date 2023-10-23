mod let_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn r0_is_zero() {
        let [r0, ..] = run_virtual_machine(
            r"
            let     r0, 4095
        ",
        );
        assert_eq!(r0, 0);
    }

    #[test]
    fn with_small_enough_number_assigns() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 4095
        ",
        );
        assert_eq!(r1, 4095);
    }

    #[test]
    fn with_too_large_number_truncates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 4097
        ",
        );
        assert_eq!(r1, 1);
    }
}

mod lethi_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_too_small_number_is_zero() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            lethi   r1, 4095
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_large_enough_number_shifts() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            lethi   r1, 4097
        ",
        );
        assert_eq!(r1, 4096);
    }

    #[test]
    fn with_large_enough_number_truncates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            lethi   r1, 0xFFFFFF
        ",
        );
        assert_eq!(r1, 0xFFF000);
    }

    // TODO: Move this to assembler tests.
    // #[test]
    #[should_panic]
    fn _with_too_large_number_errors() {
        run_virtual_machine(
            r"
            lethi   r1, 0x1000000
        ",
        );
    }
}

mod shri_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            shri    r1, r1, 0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_one_halves() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            shri    r1, r1, 1
        ",
        );
        assert_eq!(r1, 8);
    }
}

mod shli_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            shli    r1, r1, 0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_one_doubles() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            shli    r1, r1, 1
        ",
        );
        assert_eq!(r1, 34);
    }
}

mod slessi_op {}

mod load_op {
    use crate::common::run_virtual_machine;
    
    #[test]
    fn loads_from_ram() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            load    r1, r2, 0
        ");
        assert_eq!(r1, 17);
    }
    
    #[test]
    fn loads_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            subi    r2, r2, 1
            load    r1, r2, 1
        ");
        assert_eq!(r1, 17);
    }

    #[test]
    fn loads_from_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            addi    r2, r2, 1
            load    r1, r2, -1
        ");
        assert_eq!(r1, 17);
    }
}

mod load2_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn loads_2_from_ram() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            load2   r1, r2, 0
        ");
        assert_eq!(r1, 4097);
    }

    #[test]
    fn loads_2_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            subi    r2, r2, 2
            load2   r1, r2, 2
        ");
        assert_eq!(r1, 4097);
    }

    #[test]
    fn loads_2_from_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            addi    r2, r2, 2
            load2   r1, r2, -2
        ");
        assert_eq!(r1, 4097);
    }
}

mod load3_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn loads_3_from_ram() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            load3   r1, r2, 0
        ");
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn loads_3_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            subi    r2, r2, 3
            load3   r1, r2, 3
        ");
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn loads_3_from_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            addi    r2, r2, 3
            load3   r1, r2, -3
        ");
        assert_eq!(r1, 0x777FFF);
    }
}