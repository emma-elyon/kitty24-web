mod global_absolute {
    use crate::common::run_virtual_machine;

    #[test]
    fn jumps_forward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            let     pc, skip_to_end
            let     r1, 34
            skip_to_end:
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn jumps_backward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            loop:
                addi    r1, r1, 1
                lessi   r0, r1, 24
                clet    pc, end
                let     pc, loop
            end:
        ",
        );
        assert_eq!(r1, 24);
    }
}

mod local_absolute {
    use crate::common::run_virtual_machine;

    #[test]
    fn jumps_forward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            main:
                let     r1, 17
                let     pc, .skip_to_end
                let     r1, 34
                .skip_to_end:
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn jumps_backward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            main:
                let     r1, 17
                .loop:
                    addi    r1, r1, 1
                    lessi   r0, r1, 24
                    clet    pc, .end
                    let     pc, .loop
            .end:
        ",
        );
        assert_eq!(r1, 24);
    }
}

mod scoped_absolute {
    use crate::common::run_virtual_machine;

    #[test]
    fn jumps_forward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            main:
                let     r1, 17
                let     pc, main.skip_to_end
                let     r1, 34
                .skip_to_end:
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn jumps_backward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            main:
                let     r1, 17
                .loop:
                    addi    r1, r1, 1
                    lessi   r0, r1, 24
                    clet    pc, main.end
                    let     pc, main.loop
            .end:
        ",
        );
        assert_eq!(r1, 24);
    }
}

mod global_relative {
    use crate::common::run_virtual_machine;

    #[test]
    fn jumps_forward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            addi    pc, pc, ~skip_to_end
            let     r1, 34
            skip_to_end:
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn jumps_backward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            let     r1, 17
            loop:
                addi    r1, r1, 1
                lessi   r0, r1, 24
                clet    pc, end
                subi    pc, pc, ~loop
            end:
        ",
        );
        assert_eq!(r1, 24);
    }
}

mod local_relative {
    use crate::common::run_virtual_machine;

    #[test]
    fn jumps_forward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            main:
                let     r1, 17
                addi    pc, pc, ~.skip_to_end
                let     r1, 34
                .skip_to_end:
        ",
        );
        assert_eq!(r1, 17);
    }

    #[test]
    fn jumps_backward() {
        let [_, r1, ..] = run_virtual_machine(
            r"
            main:
                let     r1, 17
                .loop:
                    addi    r1, r1, 1
                    lessi   r0, r1, 24
                    clet    pc, .end
                    subi    pc, pc, ~.loop
                .end:
        ",
        );
        assert_eq!(r1, 24);
    }
}

mod scoped_relative {
    use crate::common::run_virtual_machine;

    #[test]
    fn accesses_field() {
        let [_, r1, r2, r3, ..] = run_virtual_machine(
            r#"
            let     pc, main

            player:
                .x:     data 17
                .y:     data 34
                .name:  data "owo"

            main:
                let     r0, player
                lethi   r0, player
                load    r1, r0, player~.x
                load    r2, r0, player~.y
                load3   r3, r0, player~.name
        "#,
        );
        assert_eq!(r1, 17);
        assert_eq!(r2, 34);
        assert_eq!(
            String::from_utf8(r3.to_be_bytes()[1..].to_vec()).unwrap(),
            "owo"
        );
    }
}
