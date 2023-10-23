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

    #[test]
    fn with_negative_number_assigns() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, -24
            lethi   r1, -24
        ",
        );
        assert_eq!(r1, (-24_i32 << 8) as u32 >> 8);
    }

    #[test]
    fn conditionally_with_small_enough_number_assigns() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            or      r0, r0, r0
            clet    r1, 4095
            ",
        );
        assert_eq!(r1, 4095);
    }

    #[test]
    fn conditionally_with_small_enough_number_does_not_assign() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            clet    r1, 4095
            ",
        );
        assert_eq!(r1, 17);
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

    #[test]
    fn conditionally_with_large_enough_number_shifts() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            or      r0, r0, r0
            lethi   r1, 0xFFFFFF
            clethi  r1, 4097
        ",
        );
        assert_eq!(r1, 4096);
    }

    #[test]
    fn conditionally_with_large_enough_number_does_not_assign() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            lethi   r1, 0xFFFFFF
            clethi  r1, 4097
        ",
        );
        assert_eq!(r1, 0xFFF000);
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

    #[test]
    fn with_eight_demisemihemidemisemiquaverates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            shri    r1, r1, 8
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_eight_demisemihemidemisemiquaverates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            or      r0, r0, r0
            cshri   r1, r1, 8
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_eight_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            cshri   r1, r1, 8
        ",
        );
        assert_eq!(r1, 2248);
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

    #[test]
    fn with_ten_multiplies_by_1024() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2
            shli    r1, r1, 10
        ",
        );
        assert_eq!(r1, 2048);
    }

    #[test]
    fn with_twentythree_truncates_least_to_most_significant_bit() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            shli    r1, r1, 23
        ",
        );
        assert_eq!(r1, 0x800000);
    }

    #[test]
    fn with_twentyfour_truncates_to_zero() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFFF
            shli    r1, r1, 24
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn conditionally_with_ten_multiplies_by_1024() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2
            or      r0, r0, r0
            cshli   r1, r1, 10
        ",
        );
        assert_eq!(r1, 2048);
    }

    #[test]
    fn conditionally_with_ten_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2
            cshli   r1, r1, 10
        ",
        );
        assert_eq!(r1, 2);
    }
}

mod slessi_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            slessi  r1, r1, 32
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn with_less_than_zero_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, -24
            lethi   r1, -24
            slessi  r1, r1, 0
        ",
        );
        assert_eq!(r1, 1);
    }

    #[test]
    fn with_less_than_minus_12_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, -24
            lethi   r1, -24
            slessi  r1, r1, -12
        ",
        );
        assert_eq!(r1, 1);
    }

    #[test]
    fn with_equal_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            slessi  r1, r1, 24
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_greater_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            slessi  r1, r1, 16
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_equal_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            slessi  r1, r1, 24
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 32);
    }

    #[test]
    fn conditionally_with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            or      r0, r0, r0
            cslessi r1, r1, 32
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn conditionally_with_less_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            cslessi r1, r1, 32
        ",
        );
        assert_eq!(r1, 24)
    }
}

mod load_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn loads_from_ram() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            load    r1, r2, 0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn loads_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            subi    r2, r2, 1
            load    r1, r2, 1
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn loads_from_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            addi    r2, r2, 1
            load    r1, r2, -1
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn conditionally_loads_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            subi    r2, r2, 1
            or      r0, r0, r0
            cload   r1, r2, 1
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn conditionally_does_not_load_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 0
            subi    r2, r2, 1
            let     r1, 34
            cload   r1, r2, 1
        ",
        );
        assert_eq!(r1, 34);
    }
}

mod load2_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn loads_2_from_ram() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            load2   r1, r2, 0
        ",
        );
        assert_eq!(r1, 4097);
    }

    #[test]
    fn loads_2_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            subi    r2, r2, 2
            load2   r1, r2, 2
        ",
        );
        assert_eq!(r1, 4097);
    }

    #[test]
    fn loads_2_from_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            addi    r2, r2, 2
            load2   r1, r2, -2
        ",
        );
        assert_eq!(r1, 4097);
    }

    #[test]
    fn conditionally_loads_2_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            subi    r2, r2, 2
            or      r0, r0, r0
            cload2  r1, r2, 2
        ",
        );
        assert_eq!(r1, 4097);
    }

    #[test]
    fn conditionally_does_not_load_2_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 0
            subi    r2, r2, 2
            let     r1, 345
            cload2  r1, r2, 2
        ",
        );
        assert_eq!(r1, 345);
    }
}

mod load3_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn loads_3_from_ram() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            load3   r1, r2, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn loads_3_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            subi    r2, r2, 3
            load3   r1, r2, 3
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn loads_3_from_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            addi    r2, r2, 3
            load3   r1, r2, -3
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn conditionally_loads_3_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            subi    r2, r2, 3
            or      r0, r0, r0
            cload3  r1, r2, 3
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn conditionally_does_not_load_3_from_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 0
            subi    r2, r2, 3
            let     r1, 0xFFF777
            cload3  r1, r2, 3
        ",
        );
        assert_eq!(r1, 0x777);
    }
}
