use itertools::repeat_n;
use itertools::Itertools;
use std::convert::TryInto;

fn main() {
    let original_intcode = vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 27, 28, 225, 1, 113, 14, 224, 1001,
        224, -34, 224, 4, 224, 102, 8, 223, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1102, 52, 34,
        224, 101, -1768, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 223, 224, 223,
        1002, 187, 14, 224, 1001, 224, -126, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1,
        224, 223, 223, 1102, 54, 74, 225, 1101, 75, 66, 225, 101, 20, 161, 224, 101, -54, 224, 224,
        4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1101, 6, 30, 225, 2, 88,
        84, 224, 101, -4884, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 2, 224, 224, 1, 224, 223,
        223, 1001, 214, 55, 224, 1001, 224, -89, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224,
        1, 224, 223, 223, 1101, 34, 69, 225, 1101, 45, 67, 224, 101, -112, 224, 224, 4, 224, 102,
        8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1102, 9, 81, 225, 102, 81, 218, 224, 101,
        -7290, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1101, 84,
        34, 225, 1102, 94, 90, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223,
        1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 1008, 677, 677,
        224, 102, 2, 223, 223, 1005, 224, 359, 101, 1, 223, 223, 8, 226, 677, 224, 1002, 223, 2,
        223, 1006, 224, 374, 101, 1, 223, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224,
        389, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 1001, 223,
        1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 1107, 677,
        226, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 1107, 226, 226, 224, 1002,
        223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 1108, 226, 226, 224, 1002, 223, 2, 223,
        1005, 224, 464, 101, 1, 223, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 479, 101,
        1, 223, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 494, 1001, 223, 1, 223, 1007,
        226, 677, 224, 1002, 223, 2, 223, 1006, 224, 509, 1001, 223, 1, 223, 108, 226, 226, 224,
        1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 1108, 677, 226, 224, 102, 2, 223,
        223, 1006, 224, 539, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006, 224,
        554, 101, 1, 223, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 101, 1, 223,
        223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 7, 677, 226,
        224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223, 1008, 226, 226, 224, 1002, 223, 2,
        223, 1005, 224, 614, 1001, 223, 1, 223, 107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224,
        629, 101, 1, 223, 223, 7, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 644, 1001, 223, 1,
        223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 659, 101, 1, 223, 223, 108, 677,
        677, 224, 102, 2, 223, 223, 1005, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ];
    let mut intcode = original_intcode.clone();
    let result = execute_intcode(&mut intcode, 1);

    println!(
        "{}-1: {:?}",
        std::env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        result
    );

    let mut intcode = original_intcode.clone();
    let result = execute_intcode(&mut intcode, 5);

    println!(
        "{}-2: {:?}",
        std::env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        result
    );
}

fn get_intcode_result(intcode: Vec<isize>) -> isize {
    let mut local_intcode = intcode;
    execute_intcode(&mut local_intcode, 0);
    local_intcode[0]
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

fn code_to_mode(i: isize) -> Mode {
    match i {
        0 => Mode::Position,
        1 => Mode::Immediate,
        x => panic!("illegal mode {:?}", x),
    }
}

fn execute_intcode(intcode: &mut Vec<isize>, input: isize) -> isize {
    // TODO: this feels really gross for rust code, but writing and reading from the same memory
    // bank is pretty much what rust is designed to avoid, so maybe this is fine...?
    let mut output = 0;
    let mut i = 0;
    while i < intcode.len() {
        let code = intcode[i];
        let pm1 = code_to_mode(code / 100 % 10);
        let pm2 = code_to_mode(code / 1000 % 10);
        let pm3 = code_to_mode(code / 10000 % 10);
        let code = code % 100;

        //println!("{:?} {:?} {:?} {:?} ", code, pm1, pm2, pm3);

        match code {
            1 => {
                // Add
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let b = match pm2 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let c = match pm3 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                intcode[c] = intcode[a] + intcode[b];
            }
            2 => {
                // Multiply
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let b = match pm2 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let c = match pm3 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                intcode[c] = intcode[a] * intcode[b];
            }
            3 => {
                // Store input
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                intcode[a] = input;
            }
            4 => {
                // Set output
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                output = intcode[a];
            }
            5 => {
                // Jump-if-true
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let b = match pm2 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                if intcode[a] != 0 {
                    i = intcode[b] as usize - 1; // -1 to account for increment at end of loop
                }
            }
            6 => {
                // Jump-if-false
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let b = match pm2 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                if intcode[a] == 0 {
                    i = intcode[b] as usize - 1; // -1 to account for increment at end of loop
                }
            }
            7 => {
                // less than
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let b = match pm2 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let c = match pm3 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                if intcode[a] < intcode[b] {
                    intcode[c as usize] = 1;
                } else {
                    intcode[c as usize] = 0;
                }
            }
            8 => {
                // equals
                i += 1;
                let a = match pm1 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let b = match pm2 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                i += 1;
                let c = match pm3 {
                    Mode::Position => intcode[i] as usize,
                    Mode::Immediate => i,
                };
                if intcode[a] == intcode[b] {
                    intcode[c] = 1;
                } else {
                    intcode[c] = 0;
                }
            }
            99 => break,
            opcode => panic!("{:?} not allowed as opcode", opcode),
        }
        i += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new_opcodes {
        use super::*;

        #[test]
        fn example1() {
            let mut program = vec![3, 0, 4, 0, 99];
            let input = 10;
            let expected = vec![10, 0, 4, 0, 99];
            let expected_output = 10;
            let output = execute_intcode(&mut program, input);
            assert_eq!(program, expected);
        }

        #[test]
        fn example2() {
            let mut program = vec![1002, 4, 3, 4, 33];
            let expected = vec![1002, 4, 3, 4, 99];
            execute_intcode(&mut program, 0);
            assert_eq!(program, expected);
        }

        #[test]
        fn example3() {
            let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 8);
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 3);
            assert_eq!(output, 0);
        }

        #[test]
        fn example4() {
            let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 1);
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 10);
            assert_eq!(output, 0);
        }

        #[test]
        fn example5() {
            let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 8);
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 3);
            assert_eq!(output, 0);
        }

        #[test]
        fn example6() {
            let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 1);
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 10);
            assert_eq!(output, 0);
        }

        #[test]
        fn example7() {
            let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 0);
            assert_eq!(output, 0);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 10);
            assert_eq!(output, 1);
        }

        #[test]
        fn example8() {
            let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 0);
            assert_eq!(output, 0);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 10);
            assert_eq!(output, 1);
        }

        #[test]
        fn example9() {
            let program = vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 0);
            assert_eq!(output, 999);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 8);
            assert_eq!(output, 1000);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, 10);
            assert_eq!(output, 1001);
        }
    }

    #[test]
    fn test_example() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        execute_intcode(&mut program, 0);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_2() {
        let mut program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        execute_intcode(&mut program, 0);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_3() {
        let mut program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        execute_intcode(&mut program, 0);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_4() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        execute_intcode(&mut program, 0);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_5() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        execute_intcode(&mut program, 0);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_result() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(get_intcode_result(program), 3500);
    }

    #[test]
    fn test_example_2_result() {
        let program = vec![1, 0, 0, 0, 99];
        assert_eq!(get_intcode_result(program), 2);
    }

    #[test]
    fn test_example_3_result() {
        let program = vec![2, 3, 0, 3, 99];
        assert_eq!(get_intcode_result(program), 2);
    }

    #[test]
    fn test_example_4_result() {
        let program = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(get_intcode_result(program), 2);
    }

    #[test]
    fn test_example_5_result() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(get_intcode_result(program), 30);
    }
}
