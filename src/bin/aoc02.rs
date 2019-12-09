use itertools::repeat_n;
use itertools::Itertools;

fn main() {
    let original_intcode = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 5, 19, 23, 1, 23, 6, 27, 1,
        5, 27, 31, 1, 31, 6, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 43, 6, 47, 2, 6, 47, 51, 1, 5, 51,
        55, 1, 55, 13, 59, 1, 59, 10, 63, 2, 10, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75, 79,
        2, 79, 13, 83, 1, 83, 5, 87, 1, 87, 9, 91, 1, 5, 91, 95, 1, 5, 95, 99, 1, 99, 13, 103, 1,
        10, 103, 107, 1, 107, 9, 111, 1, 6, 111, 115, 2, 115, 13, 119, 1, 10, 119, 123, 2, 123, 6,
        127, 1, 5, 127, 131, 1, 5, 131, 135, 1, 135, 6, 139, 2, 139, 10, 143, 2, 143, 9, 147, 1,
        147, 6, 151, 1, 151, 13, 155, 2, 155, 9, 159, 1, 6, 159, 163, 1, 5, 163, 167, 1, 5, 167,
        171, 1, 10, 171, 175, 1, 13, 175, 179, 1, 179, 2, 183, 1, 9, 183, 0, 99, 2, 14, 0, 0,
    ];
    let mut intcode = original_intcode.clone();
    intcode[1] = 12;
    intcode[2] = 2;
    let result = get_intcode_result(intcode);

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

    for x in repeat_n(0..original_intcode.len(), 2).multi_cartesian_product() {
        let mut intcode = original_intcode.clone();
        intcode[1] = x[0];
        intcode[2] = x[1];
        let result = get_intcode_result(intcode);
        if result == 19_690_720 {
            println!(
                "{}-1: {:?}",
                std::env::current_exe()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                100 * x[0] + x[1]
            );
        }
    }
}

fn get_intcode_result(intcode: Vec<usize>) -> usize {
    let mut local_intcode = intcode;
    execute_intcode(&mut local_intcode);
    local_intcode[0]
}

fn execute_intcode(intcode: &mut Vec<usize>) {
    for i in (0..intcode.len()).step_by(4) {
        match intcode[i] {
            1 => {
                if let [a, b, c] = intcode[(i + 1)..=(i + 3)] {
                    intcode[c] = intcode[a] + intcode[b];
                }
            }
            2 => {
                if let [a, b, c] = intcode[(i + 1)..=(i + 3)] {
                    intcode[c] = intcode[a] * intcode[b];
                }
            }
            99 => break,
            opcode => panic!("{:?} not allowed as opcode", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        execute_intcode(&mut program);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_2() {
        let mut program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        execute_intcode(&mut program);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_3() {
        let mut program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        execute_intcode(&mut program);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_4() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        execute_intcode(&mut program);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_example_5() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        execute_intcode(&mut program);
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
