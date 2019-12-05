use itertools::Itertools;

fn main() {
    let result = (138241..674034)
        .filter(has_double)
        .filter(always_increments)
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

    let result = (138241..674034)
        .filter(has_exclusive_double)
        .filter(always_increments)
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

fn has_double(i: &u64) -> bool {
    (0..6)
        .map(|x| 10u64.pow(x))
        .map(|x| i / x % 10)
        .tuple_windows()
        .any(|(x, y)| x == y)
}

fn has_exclusive_double(i: &u64) -> bool {
    // TODO: I feel like there is a clearer way to do this, maybe a way to map groups of
    // consecutive duplicate, or a fold or something?
    let digits = (0..6)
        .map(|x| 10u64.pow(x))
        .map(|x| i / x % 10)
        .collect_vec();
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            if i <= 0 || digits[i] != digits[i - 1] {
                if i >= 4 || digits[i] != digits[i + 2] {
                    return true;
                }
            }
        }
    }
    return false;
}

fn always_increments(i: &u64) -> bool {
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
            assert!(has_double(&223450));
            assert!(has_double(&111111));
        }

        #[test]
        fn no() {
            assert!(!has_double(&123789));
        }
    }

    mod exclusive_double {
        use super::*;

        #[test]
        fn yes() {
            assert!(has_exclusive_double(&223450));
            assert!(has_exclusive_double(&112233));
            assert!(has_exclusive_double(&111122));
        }

        #[test]
        fn no() {
            assert!(!has_exclusive_double(&123789));
            assert!(!has_exclusive_double(&111111));
            assert!(!has_exclusive_double(&123444));
        }
    }

    mod increment {
        use super::*;

        #[test]
        fn yes() {
            assert!(always_increments(&111111));
            assert!(always_increments(&123789));
        }

        #[test]
        fn no() {
            assert!(!always_increments(&223450));
            assert!(!always_increments(&123787));
            assert!(!always_increments(&120789));
            assert!(!always_increments(&123780));
        }
    }
}
