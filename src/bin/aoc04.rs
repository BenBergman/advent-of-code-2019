use itertools::Itertools;

fn main() {
    let result = (138_241..674_034)
        .filter(|&i| has_double(i))
        .filter(|&i| always_increments(i))
        .count();

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

    let result = (138_241..674_034)
        .filter(|&i| has_exclusive_double(i))
        .filter(|&i| always_increments(i))
        .count();

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

fn has_double(i: u64) -> bool {
    (0..6)
        .map(|x| 10u64.pow(x))
        .map(|x| i / x % 10)
        .tuple_windows()
        .any(|(x, y)| x == y)
}

fn has_exclusive_double(i: u64) -> bool {
    // TODO: I feel like there is a clearer way to do this, maybe a way to map groups of
    // consecutive duplicate, or a fold or something?
    let digits = (0..6)
        .map(|x| 10u64.pow(x))
        .map(|x| i / x % 10)
        .collect_vec();
    for index in 0..5 {
        if digits[index] == digits[index + 1]
            && (index == 0 || digits[index] != digits[index - 1])
            && (index >= 4 || digits[index] != digits[index + 2])
        {
            return true;
        }
    }
    false
}

fn always_increments(i: u64) -> bool {
    (0..6)
        .map(|x| 10u64.pow(x))
        .map(|x| i / x % 10)
        .tuple_windows()
        .all(|(x, y)| x >= y)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod double {
        use super::*;

        #[test]
        fn yes() {
            assert!(has_double(223450));
            assert!(has_double(111111));
        }

        #[test]
        fn no() {
            assert!(!has_double(123789));
        }
    }

    mod exclusive_double {
        use super::*;

        #[test]
        fn yes() {
            assert!(has_exclusive_double(223450));
            assert!(has_exclusive_double(112233));
            assert!(has_exclusive_double(111122));
        }

        #[test]
        fn no() {
            assert!(!has_exclusive_double(123789));
            assert!(!has_exclusive_double(111111));
            assert!(!has_exclusive_double(123444));
        }
    }

    mod increment {
        use super::*;

        #[test]
        fn yes() {
            assert!(always_increments(111111));
            assert!(always_increments(123789));
        }

        #[test]
        fn no() {
            assert!(!always_increments(223450));
            assert!(!always_increments(123787));
            assert!(!always_increments(120789));
            assert!(!always_increments(123780));
        }
    }
}
