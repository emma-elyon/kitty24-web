mod data {
    use crate::common::run_virtual_machine;

    #[test]
    fn holds_thirty_seven() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r2, address
            lethi   r2, address
            load    r1, r2, 0
            let     r2, address.end
            lethi   r2, address.end
            or      pc, r2, r0
            address:
            data    37
            .end:
        ",
        );
        assert_eq!(r1, 37)
    }

    #[test]
    fn holds_string() {
        let [_, r1, r2, r3, r4, r5, r6, ..] = run_virtual_machine(
            r#"
            let     rA, address
            lethi   rA, address
            load    r1, rA, 0
            load    r2, rA, 1
            load    r3, rA, 2
            load    r4, rA, 3
            load    r5, rA, 4
            load    r6, rA, 5
            let     r7, address.end
            lethi   r7, address.end
            ori     pc, r7, 0
            address:
                data    "Hello~"
            .end:
        "#,
        );
        assert_eq!(
            String::from_utf8(vec![
                r1 as u8, r2 as u8, r3 as u8, r4 as u8, r5 as u8, r6 as u8
            ])
            .unwrap(),
            "Hello~"
        );
    }
}

mod data2 {
    use crate::common::run_virtual_machine;

    #[test]
    fn holds_thirty_seven_thousand() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r2, address
            lethi   r2, address
            load2   r1, r2, 0
            let     r2, address.end
            lethi   r2, address.end
            or      pc, r2, r0
            address:
            data2   37_000
            .end:
        ",
        );
        assert_eq!(r1, 37_000)
    }
}

mod data3 {
    use crate::common::run_virtual_machine;

    #[test]
    fn holds_seven_million() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r2, address
            lethi   r2, address
            load3   r1, r2, 0
            let     r2, address.end
            lethi   r2, address.end
            or      pc, r2, r0
            address:
            data3   7_000_000
            .end:
        ",
        );
        assert_eq!(r1, 7_000_000)
    }
}
