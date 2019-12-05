use itertools::Itertools;
use rayon::prelude::*;
use std::error::Error;

fn main() {
    let (a, b) = parse_input("R1000,D940,L143,D182,L877,D709,L253,U248,L301,U434,R841,U715,R701,U92,R284,U115,R223,U702,R969,U184,L992,U47,L183,U474,L437,D769,L71,U96,R14,U503,R144,U432,R948,U96,L118,D696,R684,U539,L47,D851,L943,U606,L109,D884,R157,U946,R75,U702,L414,U347,R98,D517,L963,D177,R467,D142,L845,U427,R357,D528,L836,D222,L328,U504,R237,U99,L192,D147,L544,D466,R765,U845,L267,D217,L138,U182,R226,U466,R785,U989,R55,D822,L101,U292,R78,U962,R918,U218,L619,D324,L467,U885,L658,U890,L764,D747,R369,D930,L264,D916,L696,U698,R143,U537,L922,U131,R141,D97,L76,D883,R75,D657,R859,U503,R399,U33,L510,D318,L455,U128,R146,D645,L147,D651,L388,D338,L998,U321,L982,U150,R123,U834,R913,D200,L455,D479,L38,U860,L471,U945,L946,D365,L377,U816,R988,D597,R181,D253,R744,U472,L345,U495,L187,D443,R924,D536,R847,U430,L145,D827,L152,D831,L886,D597,R699,D751,R638,D580,L488,D566,L717,D220,L965,D587,L638,D880,L475,D165,L899,U388,R326,D568,R940,U550,R788,D76,L189,D641,R629,D383,L272,D840,L441,D709,L424,U158,L831,D576,R96,D401,R425,U525,L378,D907,L645,U609,L336,D232,L259,D280,L523,U938,R190,D9,L284,U941,L254,D657,R572,U443,L850,U508,L742,D661,L977,U910,L190,U626,R140,U762,L673,U741,R317,D518,R111,U28,R598,D403,R465,D684,R79,U725,L556,U302,L367,U306,R632,D550,R89,D292,R561,D84,L923,D109,L865,D880,L387,D24,R99,U934,L41,U29,L225,D12,L818,U696,R652,U327,L69,D773,L618,U803,L433,D467,R840,D281,R161,D400,R266,D67,L205,D94,R551,U332,R938,D759,L437,D515,L480,U774,L373,U478,R963,D863,L735,U138,L580,U72,L770,U968,L594
L990,D248,L833,U137,L556,U943,R599,U481,R963,U812,L825,U421,R998,D847,R377,D19,R588,D657,R197,D354,L548,U849,R30,D209,L745,U594,L168,U5,L357,D135,R94,D686,R965,U838,R192,U428,L861,U354,R653,U543,L633,D508,R655,U575,R709,D53,L801,D709,L92,U289,L466,D875,R75,D448,R576,D972,L77,U4,L267,D727,L3,D687,R743,D830,L803,D537,L180,U644,L204,U407,R866,U886,R560,D848,R507,U470,R38,D652,R806,D283,L836,D629,R347,D679,R609,D224,L131,D616,L687,U181,R539,D829,L598,D55,L806,U208,R886,U794,L268,D365,L145,U690,R50,D698,L140,D512,L551,U845,R351,U724,R405,D245,L324,U181,L824,U351,R223,D360,L687,D640,L653,U158,R786,D962,R931,D151,R939,D34,R610,U684,L694,D283,R402,D253,R388,D195,R732,U809,R246,D571,L820,U742,L507,U967,L886,D693,L273,U558,L914,D122,R146,U788,R83,U149,R241,U616,R326,U40,L192,D845,L577,U803,L668,D443,R705,D793,R443,D883,L715,U757,R767,D360,L289,D756,R696,D236,L525,U872,L332,U203,L152,D234,R559,U191,R340,U926,L746,D128,R867,D562,L100,U445,L489,D814,R921,D286,L378,D956,L36,D998,R158,D611,L493,U542,R932,U957,R55,D608,R790,D388,R414,U670,R845,D394,L572,D612,R842,U792,R959,U7,L285,U769,L410,D940,L319,D182,R42,D774,R758,D457,R10,U82,L861,D901,L310,D217,R644,U305,R92,U339,R252,U460,R609,D486,R553,D798,R809,U552,L183,D238,R138,D147,L343,D597,L670,U237,L878,U872,R789,U268,L97,D313,R22,U343,R907,D646,L36,D516,L808,U622,L927,D982,L810,D149,R390,U101,L565,U488,L588,U426,L386,U305,R503,U227,R969,U201,L698,D850,R800,D961,R387,U632,R543,D541,R750,D174,R543,D237,R487,D932,R220");
    let result = get_min_crossing_distance(&a, &b);

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

    let result = get_shortest_wire_crossing(&a, &b);

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

fn get_shortest_wire_crossing(a: &Vec<Motion>, b: &Vec<Motion>) -> i64 {
    get_intersections_from_corners(a, b)
        .iter()
        .map(|&p| distance_to_point(a, p) + distance_to_point(b, p))
        .min()
        .unwrap()
}

fn distance_to_point(a: &Vec<Motion>, p: (i64, i64)) -> i64 {
    let mut distance = 0;
    for line in get_corners(a).as_slice().windows(2) {
        if is_horizontal(line) && p.1 == line[0].1 && x_is_between(p.0, line) {
            distance += (line[0].0 - p.0).abs();
            break;
        } else if is_vertical(line) && p.0 == line[0].0 && y_is_between(p.1, line) {
            distance += (line[0].1 - p.1).abs();
            break;
        } else if is_horizontal(line) {
            distance += (line[1].0 - line[0].0).abs();
        } else if is_vertical(line) {
            distance += (line[1].1 - line[0].1).abs();
        } else {
            panic!("We shouldn't be able to get here")
        }
    }

    distance
}

fn get_min_crossing_distance(a: &Vec<Motion>, b: &Vec<Motion>) -> i64 {
    get_intersections_from_corners(a, b)
        .iter()
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap()
}

fn get_intersections(a: &Vec<Motion>, b: &Vec<Motion>) -> Vec<(i64, i64)> {
    let a_points = get_points(a);
    let b_points = get_points(b);

    // TODO: this is crazy slow, even when parallel
    // maybe switch to incrementing a hashmap and then finding entries with value of 2?
    // Another approach is finding corners instead of all points, then take all pairs of points for
    // a wire and checking if it would cross the other wire anywhere...
    a_points
        .par_iter()
        .cloned()
        .filter(|x| b_points.contains(x))
        .collect()
}

fn get_points(wire: &Vec<Motion>) -> Vec<(i64, i64)> {
    let mut p = (0, 0);
    wire.iter()
        .flat_map(|motion| {
            let new_points = match motion {
                Motion::Left(i) => ((p.0 - i)..p.0)
                    .map(|x| (x, p.1))
                    .rev()
                    .collect::<Vec<(i64, i64)>>(),
                Motion::Right(i) => ((p.0 + 1)..=(p.0 + i))
                    .map(|x| (x, p.1))
                    .collect::<Vec<(i64, i64)>>(),
                Motion::Up(i) => ((p.1 + 1)..=(p.1 + i))
                    .map(|y| (p.0, y))
                    .collect::<Vec<(i64, i64)>>(),
                Motion::Down(i) => ((p.1 - i)..p.1)
                    .map(|y| (p.0, y))
                    .rev()
                    .collect::<Vec<(i64, i64)>>(),
            };
            p = *new_points.clone().last().unwrap();
            new_points
        })
        .collect()
}

fn get_intersections_from_corners(a: &Vec<Motion>, b: &Vec<Motion>) -> Vec<(i64, i64)> {
    let a_points = get_corners(a);
    let b_points = get_corners(b);
    let b_lines = b_points.as_slice().windows(2);

    let aoeu = a_points
        .as_slice()
        .windows(2)
        .flat_map(|l| {
            b_lines
                .clone()
                .filter_map(move |l2| get_line_intersection(l, l2))
        })
        .collect::<Vec<(i64, i64)>>();

    aoeu
}

fn get_line_intersection(a: &[(i64, i64)], b: &[(i64, i64)]) -> Option<(i64, i64)> {
    if is_vertical(a) && is_horizontal(b) && x_is_between(a[0].0, b) && y_is_between(b[0].1, a) {
        // vertical first line, horizontal second line
        Some((a[0].0, b[0].1))
    } else if is_horizontal(a)
        && is_vertical(b)
        && x_is_between(b[0].0, a)
        && y_is_between(a[0].1, b)
    {
        // horizontal first line, vertical second line
        Some((b[0].0, a[0].1))
    } else {
        None
    }
}

fn is_vertical(a: &[(i64, i64)]) -> bool {
    a[0].0 == a[1].0
}

fn is_horizontal(a: &[(i64, i64)]) -> bool {
    a[0].1 == a[1].1
}

fn x_is_between(x: i64, a: &[(i64, i64)]) -> bool {
    x > a[0].0.min(a[1].0) && x < a[1].0.max(a[0].0)
}

fn y_is_between(y: i64, a: &[(i64, i64)]) -> bool {
    y > a[0].1.min(a[1].1) && y < a[1].1.max(a[0].1)
}

fn get_corners(wire: &Vec<Motion>) -> Vec<(i64, i64)> {
    let mut p = (0, 0);
    std::iter::once(p)
        .chain(wire.iter().map(|motion| {
            let new_point = match motion {
                Motion::Left(i) => ((p.0 - i), p.1),
                Motion::Right(i) => ((p.0 + i), p.1),
                Motion::Up(i) => (p.0, (p.1 + i)),
                Motion::Down(i) => (p.0, (p.1 - i)),
            };
            p = new_point.clone();
            new_point
        }))
        .collect()
}

#[derive(Debug, PartialEq)]
enum Motion {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

fn str_to_motion(string: &str) -> Result<Motion, Box<dyn Error>> {
    let mut chars = string.chars();
    chars.next();
    let num = chars.as_str().parse()?;
    assert!(num > 0); // TODO: return an error?

    if string.starts_with("L") {
        Ok(Motion::Left(num))
    } else if string.starts_with("R") {
        Ok(Motion::Right(num))
    } else if string.starts_with("U") {
        Ok(Motion::Up(num))
    } else if string.starts_with("D") {
        Ok(Motion::Down(num))
    } else {
        /* TODO: it would be nice if we could return an error, but I think I need to create a
         * custom error type then, and ain't nobody got time for that
        Err(Box::new(format!(
            "Unknown direction {:?}",
            string.chars().next().unwrap()
        )))
            */
        panic!("Unknown direction {:?}", string.chars().next().unwrap());
    }
}

fn parse_input(input: &str) -> (Vec<Motion>, Vec<Motion>) {
    input
        .split("\n")
        .map(|x| x.split(",").map(|x| str_to_motion(x).unwrap()).collect())
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod examples {
        use super::*;

        #[test]
        fn ex0() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            assert_eq!(get_min_crossing_distance(&a, &b), 6);
        }

        #[test]
        fn ex1() {
            let (a, b) = parse_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
            );
            assert_eq!(get_min_crossing_distance(&a, &b), 159);
        }

        #[test]
        fn ex2() {
            let (a, b) = parse_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            );
            assert_eq!(get_min_crossing_distance(&a, &b), 135);
        }
    }

    mod part_2 {
        use super::*;

        #[test]
        fn example() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            assert_eq!(get_shortest_wire_crossing(&a, &b), 30);
        }
    }

    mod intersections {
        use super::*;

        #[test]
        fn test() {
            let (a, b) = parse_input("R1,U8,R10\nU1,R4,U10");
            assert_eq!(get_intersections(&a, &b), vec!((1, 1), (4, 8)));
        }

        #[test]
        fn from_example() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            assert_eq!(get_intersections(&a, &b), vec![(6, 5), (3, 3)]);
        }
    }

    mod points {
        use super::*;

        #[test]
        fn basic() {
            let (a, b) = parse_input("R1,U8,R10\nU1,R4,U10");
            assert_eq!(
                get_points(&a),
                vec![
                    (1, 0),
                    (1, 1),
                    (1, 2),
                    (1, 3),
                    (1, 4),
                    (1, 5),
                    (1, 6),
                    (1, 7),
                    (1, 8),
                    (2, 8),
                    (3, 8),
                    (4, 8),
                    (5, 8),
                    (6, 8),
                    (7, 8),
                    (8, 8),
                    (9, 8),
                    (10, 8),
                    (11, 8)
                ]
            );
        }

        #[test]
        fn from_example() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            assert_eq!(
                get_points(&a),
                vec![
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (5, 0),
                    (6, 0),
                    (7, 0),
                    (8, 0),
                    (8, 1),
                    (8, 2),
                    (8, 3),
                    (8, 4),
                    (8, 5),
                    (7, 5),
                    (6, 5),
                    (5, 5),
                    (4, 5),
                    (3, 5),
                    (3, 4),
                    (3, 3),
                    (3, 2)
                ]
            );
            assert_eq!(
                get_points(&b),
                vec![
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5),
                    (0, 6),
                    (0, 7),
                    (1, 7),
                    (2, 7),
                    (3, 7),
                    (4, 7),
                    (5, 7),
                    (6, 7),
                    (6, 6),
                    (6, 5),
                    (6, 4),
                    (6, 3),
                    (5, 3),
                    (4, 3),
                    (3, 3),
                    (2, 3)
                ]
            );
        }
    }

    mod intersections_from_corners {
        use super::*;

        #[test]
        fn test() {
            let (a, b) = parse_input("R1,U8,R10\nU1,R4,U10");
            assert_eq!(get_intersections_from_corners(&a, &b), vec!((1, 1), (4, 8)));
        }

        #[test]
        fn from_example() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            assert_eq!(get_intersections_from_corners(&a, &b), vec![(6, 5), (3, 3)]);
        }
    }

    mod corners {
        use super::*;

        #[test]
        fn basic() {
            let (a, b) = parse_input("R1,U8,R10\nU1,R4,U10");
            assert_eq!(get_corners(&a), vec![(0, 0), (1, 0), (1, 8), (11, 8)]);
        }

        #[test]
        fn from_example() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            assert_eq!(
                get_corners(&a),
                vec![(0, 0), (8, 0), (8, 5), (3, 5), (3, 2)]
            );
            assert_eq!(
                get_corners(&b),
                vec![(0, 0), (0, 7), (6, 7), (6, 3), (2, 3)]
            );
        }
    }

    mod line_intersection {
        use super::*;

        #[test]
        fn none_parallel() {
            let a = vec![(1, 0), (1, 5)];
            let b = vec![(7, 0), (7, 2)];

            assert_eq!(get_line_intersection(&a, &b), None);
        }

        #[test]
        fn none_too_short() {
            let a = vec![(1, 0), (1, 5)];
            let b = vec![(0, 7), (7, 7)];

            assert_eq!(get_line_intersection(&a, &b), None);
        }

        #[test]
        fn basic() {
            let a = vec![(1, 0), (1, 5)];
            let b = vec![(0, 2), (7, 2)];

            assert_eq!(get_line_intersection(&a, &b), Some((1, 2)));
            assert_eq!(get_line_intersection(&b, &a), Some((1, 2)));
        }
    }

    mod between {
        use super::*;

        #[test]
        fn yes() {
            assert!(x_is_between(3, &vec![(0, 0), (5, 0)]));
            assert!(x_is_between(3, &vec![(5, 0), (0, 0)]));
            assert!(y_is_between(3, &vec![(0, 0), (0, 5)]));
            assert!(y_is_between(3, &vec![(0, 5), (0, 0)]));
        }

        #[test]
        fn no() {
            assert!(!x_is_between(13, &vec![(0, 0), (5, 0)]));
            assert!(!x_is_between(13, &vec![(5, 0), (0, 0)]));
            assert!(!y_is_between(13, &vec![(0, 0), (0, 5)]));
            assert!(!y_is_between(13, &vec![(0, 5), (0, 0)]));
        }
    }

    mod input_parser {
        use super::*;

        #[test]
        fn basic_test() {
            let input = "L1,R2,U3,D4\nR8,L8,D9,D3";
            assert_eq!(
                parse_input(input),
                (
                    vec![
                        Motion::Left(1),
                        Motion::Right(2),
                        Motion::Up(3),
                        Motion::Down(4),
                    ],
                    vec![
                        Motion::Right(8),
                        Motion::Left(8),
                        Motion::Down(9),
                        Motion::Down(3),
                    ],
                )
            );
        }

        #[test]
        fn single_parse() {
            assert_eq!(str_to_motion("L42").unwrap(), Motion::Left(42));
            assert_eq!(str_to_motion("R987").unwrap(), Motion::Right(987));
            assert_eq!(str_to_motion("U1").unwrap(), Motion::Up(1));
            assert_eq!(str_to_motion("D2").unwrap(), Motion::Down(2));
        }
    }

    mod distance_to_point {
        use super::*;

        #[test]
        fn example() {
            let (a, b) = parse_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            );
            let p = (6, 5);
            assert_eq!(distance_to_point(&a, p), 15);
            assert_eq!(distance_to_point(&b, p), 15);
            let p = (3, 3);
            assert_eq!(distance_to_point(&a, p), 20);
            assert_eq!(distance_to_point(&b, p), 20);
        }
    }
}
