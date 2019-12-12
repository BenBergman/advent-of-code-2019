use permutohedron::Heap;

fn main() {
    let original_intcode = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 59, 84, 93, 110, 191, 272, 353, 434, 99999, 3,
        9, 101, 2, 9, 9, 102, 3, 9, 9, 1001, 9, 5, 9, 102, 4, 9, 9, 1001, 9, 4, 9, 4, 9, 99, 3, 9,
        101, 3, 9, 9, 102, 5, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 4, 9, 1002, 9, 2, 9, 101, 2, 9, 9,
        102, 2, 9, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9,
        1001, 9, 5, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2,
        9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 99,
    ];
    let mut intcode = original_intcode.clone();
    let result = find_max_thrusters(intcode);

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

    /*
    let mut intcode = original_intcode;
    let result = execute_intcode(&mut intcode, [5].iter());

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
    */
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

fn execute_intcode<'a, I>(intcode: &mut [isize], mut input: I) -> isize
where
    I: Iterator<Item = &'a isize>,
{
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
                intcode[a] = *input.next().unwrap();
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

fn find_max_thrusters(intcode: Vec<isize>) -> (isize, [isize; 5]) {
    //Permutation::from_vec(vec![0, 1, 2, 3, 4]).inspect(|x| println!("{:?}", x));
    Heap::new(&mut [0, 1, 2, 3, 4])
        .map(|phases| (get_thrust(intcode.clone(), &phases), phases))
        .max()
        .unwrap()
}

fn get_thrust(intcode: Vec<isize>, phases: &[isize]) -> isize {
    phases
        .iter()
        .fold(0, |acc, &phase| get_amp_value(intcode.clone(), phase, acc))
}

fn get_amp_value(intcode: Vec<isize>, phase: isize, input: isize) -> isize {
    let mut program = intcode.clone();
    execute_intcode(&mut program, [phase, input].iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_intcode_result(intcode: Vec<isize>) -> isize {
        // Only really needed for the problem from the first intcode day
        let mut local_intcode = intcode;
        execute_intcode(&mut local_intcode, [0].iter());
        local_intcode[0]
    }

    mod max_thrusters {
        use super::*;

        #[test]
        fn example1() {
            let program = vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ];
            let max_thrusters = find_max_thrusters(program);
            assert_eq!(max_thrusters, (43210, [4, 3, 2, 1, 0]));
        }

        #[test]
        fn get_thrust_ex1() {
            let program = vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ];
            let thrust = get_thrust(program, &[4, 3, 2, 1, 0]);
            assert_eq!(thrust, 43210);
        }

        #[test]
        fn example2() {
            let program = vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ];
            let max_thrusters = find_max_thrusters(program);
            assert_eq!(max_thrusters, (54321, [0, 1, 2, 3, 4]));
        }

        #[test]
        fn example3() {
            let program = vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ];
            let max_thrusters = find_max_thrusters(program);
            assert_eq!(max_thrusters, (65210, [1, 0, 4, 3, 2]));
        }
    }

    mod new_opcodes {
        use super::*;

        #[test]
        fn example1() {
            let mut program = vec![3, 0, 4, 0, 99];
            let input = [10];
            let expected = vec![10, 0, 4, 0, 99];
            execute_intcode(&mut program, input.iter());
            assert_eq!(program, expected);
        }

        #[test]
        fn example2() {
            let mut program = vec![1002, 4, 3, 4, 33];
            let expected = vec![1002, 4, 3, 4, 99];
            execute_intcode(&mut program, [0].iter());
            assert_eq!(program, expected);
        }

        #[test]
        fn example3() {
            let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [8].iter());
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [3].iter());
            assert_eq!(output, 0);
        }

        #[test]
        fn example4() {
            let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [1].iter());
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [10].iter());
            assert_eq!(output, 0);
        }

        #[test]
        fn example5() {
            let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [8].iter());
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [3].iter());
            assert_eq!(output, 0);
        }

        #[test]
        fn example6() {
            let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [1].iter());
            assert_eq!(output, 1);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [10].iter());
            assert_eq!(output, 0);
        }

        #[test]
        fn example7() {
            let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [0].iter());
            assert_eq!(output, 0);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [10].iter());
            assert_eq!(output, 1);
        }

        #[test]
        fn example8() {
            let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [0].iter());
            assert_eq!(output, 0);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [10].iter());
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
            let output = execute_intcode(&mut run_program, [0].iter());
            assert_eq!(output, 999);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [8].iter());
            assert_eq!(output, 1000);
            let mut run_program = program.clone();
            let output = execute_intcode(&mut run_program, [10].iter());
            assert_eq!(output, 1001);
        }
    }

    #[test]
    fn test_example() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        execute_intcode(&mut program, [0].iter());
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_2() {
        let mut program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        execute_intcode(&mut program, [0].iter());
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_3() {
        let mut program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        execute_intcode(&mut program, [0].iter());
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_4() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        execute_intcode(&mut program, [0].iter());
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_5() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        execute_intcode(&mut program, [0].iter());
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
