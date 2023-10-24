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
    fn with_less_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            slessi  r0, r1, 48
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn with_greater_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            slessi  r0, r1, 12
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
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

mod ashr_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            ashr    r1, r1, r0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_one_halves() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            let     r2, 1
            ashr    r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn with_eight_demisemihemidemisemiquaverates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            let     r2, 8
            ashr    r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_eight_demisemihemidemisemiquaverates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            let     r2, 8
            or      r0, r0, r0
            cashr   r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_eight_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            let     r2, 8
            cashr   r1, r1, r2
        ",
        );
        assert_eq!(r1, 2248);
    }

    #[test]
    fn of_negative_eight_quarters() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, -8
            lethi   r1, -8
            let     r2, 2
            ashr    r1, r1, r2
        ",
        );
        assert_eq!(r1, ((-8_i32 << 8) >> 2) as u32 >> 8);
    }
}

mod rol_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            rol     r1, r1, r0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_twenty_four_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            let     r2, 24
            rol     r1, r1, r2
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_negative_twenty_four_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            let     r2, -24
            lethi   r2, -24
            rol     r1, r1, r2
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_four_rotates_left() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            lethi   r1, 0xABCDEF
            let     r2, 4
            rol     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0xBCDEFA);
    }

    #[test]
    fn with_negative_four_rotates_right() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            lethi   r1, 0xABCDEF
            let     r2, -4
            lethi   r2, -4
            rol     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0xFABCDE);
    }

    #[test]
    fn with_fifty_quadruples() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 5
            let     r2, 50
            rol     r1, r1, r2
        ",
        );
        assert_eq!(r1, 20);
    }

    #[test]
    fn with_negative_fifty_quarters() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 20
            let     r2, -50
            lethi   r2, -50
            rol     r1, r1, r2
        ",
        );
        assert_eq!(r1, 5);
    }

    #[test]
    fn conditionally_with_four_rotates_left() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            lethi   r1, 0xABCDEF
            let     r2, 4
            or      r0, r0, r0
            crol    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0xBCDEFA);
    }

    #[test]
    fn conditionally_with_four_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            lethi   r1, 0xABCDEF
            let     r2, 4
            crol    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0xABCDEF);
    }
}

mod shr_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            shr     r1, r1, r0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_one_halves() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            let     r2, 1
            shr     r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn with_eight_demisemihemidemisemiquaverates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            let     r2, 8
            shr     r1, r1, r2
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
            let     r2, 8
            cshr    r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_eight_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2248
            let     r2, 8
            cshr    r1, r1, r2
        ",
        );
        assert_eq!(r1, 2248);
    }
}

mod shl_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            shl     r1, r1, r0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn with_one_doubles() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            let     r2, 1
            shl     r1, r1, r2
        ",
        );
        assert_eq!(r1, 34);
    }

    #[test]
    fn with_ten_multiplies_by_1024() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2
            let     r2, 10
            shl     r1, r1, r2
        ",
        );
        assert_eq!(r1, 2048);
    }

    #[test]
    fn with_twentythree_truncates_least_to_most_significant_bit() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            let     r2, 23
            shl     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0x800000);
    }

    #[test]
    fn with_twentyfour_truncates_to_zero() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFFF
            let     r2, 24
            shl     r1, r1, r2
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
            let     r2, 10
            cshl    r1, r1, r2
        ",
        );
        assert_eq!(r1, 2048);
    }

    #[test]
    fn conditionally_with_ten_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 2
            let     r2, 10
            cshl    r1, r1, r2
        ",
        );
        assert_eq!(r1, 2);
    }
}

mod sless_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 32
            sless   r1, r1, r2
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
            sless   r1, r1, r0
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
            let     r2, -12
            lethi   r2, -12
            sless   r1, r1, r2
        ",
        );
        assert_eq!(r1, 1);
    }

    #[test]
    fn with_equal_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 24
            sless   r1, r1, r2
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_greater_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 16
            sless   r1, r1, r2
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_equal_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 24
            sless   r1, r1, r2
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 32);
    }

    #[test]
    fn with_less_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 48
            sless   r0, r1, r2
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn with_greater_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 12
            sless   r0, r1, r2
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn conditionally_with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            or      r0, r0, r0
            let     r2, 32
            csless  r1, r1, r2
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn conditionally_with_less_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 32
            csless  r1, r1, r2
        ",
        );
        assert_eq!(r1, 24)
    }
}

mod store_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn stores_in_ram() {
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
    fn stores_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, 1
            addi    r2, r2, 1
            load    r1, r2, 0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn stores_in_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            store   r2, r3, -1
            subi    r2, r2, 1
            load    r1, r2, 0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn conditionally_stores_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            or      r0, r0, r0
            cstore  r2, r3, 1
            addi    r2, r2, 1
            load    r1, r2, 0
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn conditionally_does_not_store_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 17
            let     r2, 0x800000
            lethi   r2, 0x800000
            cstore  r2, r3, 1
            addi    r2, r2, 1
            let     r1, 34
            load    r1, r2, 0
        ",
        );
        assert_eq!(r1, 0);
    }
}

mod store2_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn stores_2_in_ram() {
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
    fn stores_2_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, 2
            addi    r2, r2, 2
            load2   r1, r2, 0
        ",
        );
        assert_eq!(r1, 4097);
    }

    #[test]
    fn stores_2_in_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            store2  r2, r3, -2
            subi    r2, r2, 2
            load2   r1, r2, 0
        ",
        );
        assert_eq!(r1, 4097);
    }

    #[test]
    fn conditionally_stores_2_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 4097
            lethi   r3, 4097
            let     r2, 0x800000
            lethi   r2, 0x800000
            or      r0, r0, r0
            cstore2 r2, r3, 2
            addi    r2, r2, 2
            load2   r1, r2, 0
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
            cstore2 r2, r3, 0
            subi    r2, r2, 2
            let     r1, 345
            load2   r1, r2, 2
        ",
        );
        assert_eq!(r1, 0);
    }
}

mod store3_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn stores_3_in_ram() {
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
    fn stores_3_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, 3
            addi    r2, r2, 3
            load3   r1, r2, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn stores_3_in_ram_with_negative_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            store3  r2, r3, -3
            subi    r2, r2, 3
            load3   r1, r2, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn conditionally_stores_3_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            or      r0, r0, r0
            cstore3 r2, r3, 3
            addi    r2, r2, 3
            load3   r1, r2, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn conditionally_does_not_store_3_in_ram_with_offset() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r3, 0x777FFF
            lethi   r3, 0x777FFF
            let     r2, 0x800000
            lethi   r2, 0x800000
            cstore3 r2, r3, 3
            addi    r2, r2, 3
            let     r1, 0xFFF777
            load3   r1, r2, 0
        ",
        );
        assert_eq!(r1, 0);
    }
}

mod ori_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            ori     r1, r1, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn with_0o77_sets_all() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            ori     r1, r1, 0o77
        ",
        );
        assert_eq!(r1, 0o77);
    }

    #[test]
    fn with_fifteen_sets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            ori     r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10101111);
    }

    #[test]
    fn conditionally_with_fifteen_sets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            cori    r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10101111);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            cori    r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_zero_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 34
            ori     r0, r0, 0
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod nori_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_negates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010_10101010_10101010
            lethi   r1, 0b10101010_10101010_10101010
            nori    r1, r1, 0
        ",
        );
        assert_eq!(r1, 0b01010101_01010101_01010101);
    }

    #[test]
    fn with_0o77_unsets_all() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0o77770000
            lethi   r1, 0o77770000
            nori    r1, r1, 0o77
        ",
        );
        assert_eq!(r1, 0o7700);
    }

    #[test]
    fn with_fifteen_unsets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            nori    r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b11111111_11111111_01010000);
    }

    #[test]
    fn conditionally_with_fifteen_sets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            cnori   r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b11111111_11111111_01010000);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            cnori   r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_0xffffff_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFF0
            lethi   r1, 0xFFFFF0
            nori    r1, r1, 0xF
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod andi_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_zeroes() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0o77777777
            lethi   r1, 0o77777777
            andi    r1, r1, 0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_0o77_keeps_all() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFF5D
            lethi   r1, 0xFFFF5D
            andi    r1, r1, 0o77
        ",
        );
        assert_eq!(r1, 0xFFFF5D & 0o77);
    }

    #[test]
    fn with_fifteen_keeps_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            andi     r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b00001010);
    }

    #[test]
    fn conditionally_with_fifteen_keeps_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            candi   r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b00001010);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            candi   r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_zero_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 34
            andi    r0, r0, 0
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod xori_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            lethi   r1, 0xABCDEF
            xori    r1, r1, 0
        ",
        );
        assert_eq!(r1, 0xABCDEF);
    }

    #[test]
    fn with_0o77_flips_first_six() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            xori    r1, r1, 0o77
        ",
        );
        assert_eq!(r1, 0b10010101);
    }

    #[test]
    fn with_fifteen_flips_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            xori    r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10100101);
    }

    #[test]
    fn conditionally_with_fifteen_keeps_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            cxori   r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10100101);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            cxori   r1, r1, 15
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_equal_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 34
            xori    r0, r1, 34
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod lessi_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            lessi  r1, r1, 32
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn with_less_than_zero_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, -24
            lethi   r1, -24
            lessi   r1, r1, 0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_less_than_zero_is_false_for_large_number() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFFF
            lethi   r1, 0xFFFFFF
            lessi   r1, r1, 0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_equal_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            lessi   r1, r1, 24
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_greater_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            lessi   r1, r1, 16
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_equal_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            lessi   r1, r1, 24
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 32);
    }

    #[test]
    fn with_less_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            lessi   r0, r1, 48
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn with_greater_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            lessi   r0, r1, 12
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn conditionally_with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            or      r0, r0, r0
            clessi  r1, r1, 32
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn conditionally_with_less_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            clessi  r1, r1, 32
        ",
        );
        assert_eq!(r1, 24)
    }
}

mod addi_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            addi    r1, r1, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn with_0o77_adds_63() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            addi    r1, r1, 0o77
        ",
        );
        assert_eq!(r1, 86);
    }

    #[test]
    fn with_fifteen_adds_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            addi    r1, r1, 15
        ",
        );
        assert_eq!(r1, 38);
    }

    #[test]
    fn conditionally_with_fifteen_adds_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            or      r0, r0, r0
            caddi   r1, r1, 15
        ",
        );
        assert_eq!(r1, 38);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            caddi   r1, r1, 15
        ",
        );
        assert_eq!(r1, 23);
    }

    #[test]
    fn with_overflow_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFFF
            lethi   r1, 0xFFFFFF
            addi    r0, r1, 1
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod subi_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            subi    r1, r1, 0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn with_0o77_subtracts_63() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            subi    r1, r1, 0o77
        ",
        );
        assert_eq!(r1, (23 - 63) as u32 & 0xFFFFFF);
    }

    #[test]
    fn with_fifteen_subtracts_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            subi    r1, r1, 15
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_fifteen_subtracts_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            or      r0, r0, r0
            csubi   r1, r1, 15
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            csubi   r1, r1, 15
        ",
        );
        assert_eq!(r1, 23);
    }

    #[test]
    fn with_underflow_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0
            subi    r0, r1, 1
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod muli_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_zeroes() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            muli    r1, r1, 0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_0o77_multiplies_by_63() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            muli    r1, r1, 0o77
        ",
        );
        assert_eq!(r1, 23 * 63);
    }

    #[test]
    fn with_fifteen_multiplies_by_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            muli    r1, r1, 15
        ",
        );
        assert_eq!(r1, 23 * 15);
    }

    #[test]
    fn conditionally_with_fifteen_multiplies_by_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            or      r0, r0, r0
            cmuli   r1, r1, 15
        ",
        );
        assert_eq!(r1, 23 * 15);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            cmuli   r1, r1, 15
        ",
        );
        assert_eq!(r1, 23);
    }

    #[test]
    fn with_overflow_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x100000
            lethi   r1, 0x100000
            muli    r0, r1, 16
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod or_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            or      r1, r1, r0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn with_0o77_sets_all() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 0o77
            or      r1, r1, r2
        ",
        );
        assert_eq!(r1, 0o77);
    }

    #[test]
    fn with_fifteen_sets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            or      r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10101111);
    }

    #[test]
    fn conditionally_with_fifteen_sets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            let     r2, 15
            cor     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10101111);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            cor     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_zero_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 34
            or      r0, r0, r0
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod nor_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_negates() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010_10101010_10101010
            lethi   r1, 0b10101010_10101010_10101010
            nor     r1, r1, r0
        ",
        );
        assert_eq!(r1, 0b01010101_01010101_01010101);
    }

    #[test]
    fn with_0o77_unsets_all() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0o77770000
            lethi   r1, 0o77770000
            let     r2, 0o77
            nor     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0o7700);
    }

    #[test]
    fn with_fifteen_unsets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            nor     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b11111111_11111111_01010000);
    }

    #[test]
    fn conditionally_with_fifteen_sets_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            or      r0, r0, r0
            cnor    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b11111111_11111111_01010000);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            cnor    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_0xffffff_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFF0
            lethi   r1, 0xFFFFF0
            let     r2, 0xF
            nor     r1, r1, r2
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod and_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_zeroes() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0o77777777
            lethi   r1, 0o77777777
            and     r1, r1, r0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_0o77_keeps_all() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFF5D
            lethi   r1, 0xFFFF5D
            let     r2, 0o77
            and     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0xFFFF5D & 0o77);
    }

    #[test]
    fn with_fifteen_keeps_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            and     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b00001010);
    }

    #[test]
    fn conditionally_with_fifteen_keeps_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            let     r2, 15
            cand    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b00001010);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            cand    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_zero_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 34
            and     r0, r0, r0
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod xor_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xABCDEF
            lethi   r1, 0xABCDEF
            xor     r1, r1, r0
        ",
        );
        assert_eq!(r1, 0xABCDEF);
    }

    #[test]
    fn with_0o77_flips_first_six() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 0o77
            xor     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10010101);
    }

    #[test]
    fn with_fifteen_flips_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            xor     r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10100101);
    }

    #[test]
    fn conditionally_with_fifteen_keeps_first_four() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            or      r0, r0, r0
            let     r2, 15
            cxor    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10100101);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0b10101010
            let     r2, 15
            cxor    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0b10101010);
    }

    #[test]
    fn with_equal_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 34
            let     r2, 34
            xor     r0, r1, r2
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod less_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 32
            less    r1, r1, r2
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn with_less_than_zero_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, -24
            lethi   r1, -24
            less    r1, r1, r0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_less_than_zero_is_false_for_large_number() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFFF
            lethi   r1, 0xFFFFFF
            less    r1, r1, r0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_equal_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 24
            less    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_greater_is_false() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 16
            less    r1, r1, r2
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_equal_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 24
            less    r1, r1, r2
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 32);
    }

    #[test]
    fn with_less_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 48
            less    r0, r1, r2
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn with_greater_does_not_set_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 12
            less    r0, r1, r2
            clet    r1, 32
        ",
        );
        assert_eq!(r1, 24);
    }

    #[test]
    fn conditionally_with_less_is_true() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            or      r0, r0, r0
            let     r2, 32
            cless   r1, r1, r2
        ",
        );
        assert_eq!(r1, 1)
    }

    #[test]
    fn conditionally_with_less_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 24
            let     r2, 32
            cless   r1, r1, r2
        ",
        );
        assert_eq!(r1, 24)
    }
}

mod add_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            add     r1, r1, r0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn with_0o77_adds_63() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 0o77
            add     r1, r1, r2
        ",
        );
        assert_eq!(r1, 86);
    }

    #[test]
    fn with_fifteen_adds_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 15
            add     r1, r1, r2
        ",
        );
        assert_eq!(r1, 38);
    }

    #[test]
    fn conditionally_with_fifteen_adds_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            or      r0, r0, r0
            let     r2, 15
            cadd    r1, r1, r2
        ",
        );
        assert_eq!(r1, 38);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 15
            cadd    r1, r1, r2
        ",
        );
        assert_eq!(r1, 23);
    }

    #[test]
    fn with_overflow_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0xFFFFFF
            lethi   r1, 0xFFFFFF
            let     r2, 1
            add     r0, r1, r2
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod sub_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_changes_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            sub     r1, r1, r0
        ",
        );
        assert_eq!(r1, 0x777FFF);
    }

    #[test]
    fn with_0o77_subtracts_63() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 0o77
            sub     r1, r1, r2
        ",
        );
        assert_eq!(r1, (23 - 63) as u32 & 0xFFFFFF);
    }

    #[test]
    fn with_fifteen_subtracts_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 15
            sub     r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_fifteen_subtracts_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            or      r0, r0, r0
            let     r2, 15
            csub    r1, r1, r2
        ",
        );
        assert_eq!(r1, 8);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 15
            csub    r1, r1, r2
        ",
        );
        assert_eq!(r1, 23);
    }

    #[test]
    fn with_underflow_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0
            let     r2, 1
            sub     r0, r1, r2
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}

mod mul_op {
    use crate::common::run_virtual_machine;

    #[test]
    fn with_zero_zeroes() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x777FFF
            lethi   r1, 0x777FFF
            mul     r1, r1, r0
        ",
        );
        assert_eq!(r1, 0);
    }

    #[test]
    fn with_0o77_multiplies_by_63() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 0o77
            mul     r1, r1, r2
        ",
        );
        assert_eq!(r1, 23 * 63);
    }

    #[test]
    fn with_fifteen_multiplies_by_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 15
            mul     r1, r1, r2
        ",
        );
        assert_eq!(r1, 23 * 15);
    }

    #[test]
    fn conditionally_with_fifteen_multiplies_by_15() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            or      r0, r0, r0
            let     r2, 15
            cmul    r1, r1, r2
        ",
        );
        assert_eq!(r1, 23 * 15);
    }

    #[test]
    fn conditionally_with_fifteen_does_nothing() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 23
            let     r2, 15
            cmul    r1, r1, r2
        ",
        );
        assert_eq!(r1, 23);
    }

    #[test]
    fn with_overflow_sets_condition() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 0x100000
            lethi   r1, 0x100000
            let     r2, 16
            mul     r0, r1, r2
            clet    r1, 17
        ",
        );
        assert_eq!(r1, 17)
    }
}
