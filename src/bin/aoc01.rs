fn main() {
    let masses = vec![
        86608, 97271, 51200, 149_104, 86406, 97844, 74380, 125_817, 56605, 125_891, 63835, 131_033,
        142_276, 126_174, 78742, 113_961, 126_933, 105_209, 116_007, 88301, 89203, 109_951, 100_609, 68863,
        106_611, 86765, 50887, 80834, 126_291, 87119, 137_577, 123_005, 135_688, 66530, 106_270, 94168,
        92881, 51170, 59598, 60445, 71249, 86492, 141_475, 137_397, 149_715, 99862, 144_797, 135_188,
        133_640, 96909, 85245, 107_849, 126_123, 112_848, 76667, 112_760, 121_517, 75878, 82591, 116_926,
        56514, 131_864, 148_794, 139_636, 106_349, 76418, 83862, 142_732, 139_332, 142_236, 108_925,
        130_420, 59682, 72933, 50265, 99444, 52089, 57686, 75440, 51043, 149_777, 108_662, 146_667,
        90802, 147_235, 91776, 76203, 67766, 68173, 103_707, 54682, 145_674, 135_349, 58766, 92270,
        126_388, 111_236, 69184, 66915, 117_342,
    ];

    let result: u64 = masses.clone().into_iter().map(find_fuel_requirement).sum();
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

    let result: u64 = masses.into_iter().map(find_complete_fuel_requirement).sum();
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

fn find_fuel_requirement(mass: u64) -> u64 {
    match mass / 3 {
        x if x >= 2 => x - 2,
        _ => 0,
    }
}

fn find_complete_fuel_requirement(mass: u64) -> u64 {
    match find_fuel_requirement(mass) {
        base_fuel if base_fuel == 0 => base_fuel,
        base_fuel => base_fuel + find_complete_fuel_requirement(base_fuel),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(find_fuel_requirement(8), 0);
        assert_eq!(find_fuel_requirement(12), 2);
        assert_eq!(find_fuel_requirement(14), 2);
        assert_eq!(find_fuel_requirement(1969), 654);
        assert_eq!(find_fuel_requirement(100_756), 33583);
    }

    #[test]
    fn test_complete_fuel_requirement() {
        assert_eq!(find_complete_fuel_requirement(12), 2);
        assert_eq!(find_complete_fuel_requirement(14), 2);
        assert_eq!(find_complete_fuel_requirement(1969), 966);
        assert_eq!(find_complete_fuel_requirement(100_756), 50346);
    }
}
