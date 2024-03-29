use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;
use std::sync::Mutex;

fn main() {
    let input = fs::read_to_string("src/bin/aoc06.txt").unwrap();

    let result = OrbitMap::new(input.as_str()).total_orbits();
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

    let result = OrbitMap::new(input.as_str()).get_shortest_transfer_distance("YOU", "SAN");
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

struct Body<'a> {
    name: &'a str,
    orbit_center: Option<Rc<Mutex<Body<'a>>>>,
}

impl Body<'_> {
    fn get_orbit_count(&self) -> u64 {
        match &self.orbit_center {
            None => 0,
            Some(center) => 1 + center.lock().unwrap().get_orbit_count(),
        }
    }

    fn get_distance_to(&self, a: &str) -> u64 {
        match &self.orbit_center {
            None => 0,
            Some(center) if center.lock().unwrap().name == a => 1,
            Some(center) => 1 + center.lock().unwrap().get_distance_to(a),
        }
    }
}

impl fmt::Debug for Body<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} orbits {:?}",
            self.name,
            match &self.orbit_center {
                None => "nothing",
                Some(body) => body.lock().unwrap().name,
            }
        )
    }
}

#[derive(Debug)]
struct OrbitMap<'a> {
    bodies: HashMap<&'a str, Rc<Mutex<Body<'a>>>>,
}

impl<'a> OrbitMap<'a> {
    fn new(input: &'a str) -> Self {
        let bodies = HashMap::new();
        let mut orbit_map = OrbitMap { bodies };

        for line in input.lines() {
            let bodies = line.split(')').collect::<Vec<&str>>();
            orbit_map.add_orbit(bodies[0], bodies[1]);
        }

        orbit_map
    }

    fn add_orbit(&mut self, center: &'a str, satellite: &'a str) {
        let center_body = match self.bodies.get(center) {
            None => {
                let new_body = Rc::new(Mutex::new(Body {
                    name: center,
                    orbit_center: None,
                }));
                self.bodies.insert(center, new_body);
                self.bodies.get(center).unwrap()
            }
            Some(body) => body,
        }
        .clone();
        match self.bodies.get_mut(satellite) {
            None => {
                let new_body = Body {
                    name: satellite,
                    orbit_center: Some(center_body),
                };
                self.bodies.insert(satellite, Rc::new(Mutex::new(new_body)));
            }
            Some(body) => {
                let mut body = body.lock().unwrap();
                if body.orbit_center.is_none() {
                    body.orbit_center = Some(center_body);
                }
            }
        };
    }

    fn total_orbits(&self) -> u64 {
        self.bodies
            .values()
            .map(|x| x.lock().unwrap().get_orbit_count())
            .sum()
    }

    fn get_common_bodies(&self, a: &str, b: &str) -> Vec<&str> {
        let mut a_bodies = vec![];
        let mut current_body = match self.bodies.get(a) {
            Some(body) => Some(body.clone()),
            None => None,
        };
        while let Some(body) = current_body {
            a_bodies.push(body.lock().unwrap().name);
            current_body = body.lock().unwrap().orbit_center.clone();
        }

        let mut b_bodies = vec![];
        let mut current_body = match self.bodies.get(b) {
            Some(body) => Some(body.clone()),
            None => None,
        };
        while let Some(body) = current_body {
            b_bodies.push(body.lock().unwrap().name);
            current_body = body.lock().unwrap().orbit_center.clone();
        }

        b_bodies
            .iter()
            .copied()
            .filter(|x| a_bodies.contains(x))
            .collect()
    }

    fn get_distance_between_bodies(&self, a: &str, b: &str) -> u64 {
        self.bodies
            .get(a)
            .unwrap()
            .lock()
            .unwrap()
            .get_distance_to(b)
    }

    fn get_shortest_transfer_distance(&self, a: &str, b: &str) -> u64 {
        self.get_common_bodies(a, b)
            .iter()
            .map(|x| {
                self.get_distance_between_bodies(a, x) + self.get_distance_between_bodies(b, x) - 2
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod full {
        use super::*;

        #[test]
        fn example() {
            let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
            let orbit_map = OrbitMap::new(input);
            assert_eq!(
                orbit_map
                    .bodies
                    .get("COM")
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_orbit_count(),
                0
            );
            assert_eq!(orbit_map.total_orbits(), 42);
        }

        #[test]
        fn sub_example_1() {
            let input = "COM)B
B)C
C)D
";
            let orbit_map = OrbitMap::new(input);
            assert_eq!(
                orbit_map
                    .bodies
                    .get("D")
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_orbit_count(),
                3
            );
            assert_eq!(orbit_map.total_orbits(), 6);
        }

        #[test]
        fn sub_example_1_shuffled() {
            let input = "B)C
COM)B
C)D
";
            let orbit_map = OrbitMap::new(input);
            assert_eq!(
                orbit_map
                    .bodies
                    .get("D")
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_orbit_count(),
                3
            );
            assert_eq!(orbit_map.total_orbits(), 6);
        }

        #[test]
        fn sub_example_2() {
            let input = "COM)B
B)C
C)D
D)E
E)J
J)K
K)L
";
            let orbit_map = OrbitMap::new(input);
            assert_eq!(
                orbit_map
                    .bodies
                    .get("L")
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_orbit_count(),
                7
            );
            assert_eq!(orbit_map.total_orbits(), 28);
        }
    }

    mod body {
        use super::*;

        #[test]
        fn no_orbits() {
            let body = Body {
                name: "A",
                orbit_center: None,
            };
            assert_eq!(body.get_orbit_count(), 0);
        }

        #[test]
        fn one_orbit() {
            let center = Body {
                name: "A",
                orbit_center: None,
            };
            let body = Body {
                name: "B",
                orbit_center: Some(Rc::new(Mutex::new(center))),
            };
            assert_eq!(body.get_orbit_count(), 1);
        }

        #[test]
        fn one_orbit_distances() {
            let a = Body {
                name: "A",
                orbit_center: None,
            };
            let b = Body {
                name: "B",
                orbit_center: Some(Rc::new(Mutex::new(a))),
            };
            let c = Body {
                name: "C",
                orbit_center: Some(Rc::new(Mutex::new(b))),
            };
            assert_eq!(c.get_distance_to("B"), 1);
            assert_eq!(c.get_distance_to("A"), 2);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
            let orbit_map = OrbitMap::new(input);
            assert_eq!(
                orbit_map.get_common_bodies("YOU", "SAN"),
                vec!["D", "C", "B", "COM"]
            );
            assert_eq!(orbit_map.get_shortest_transfer_distance("YOU", "SAN"), 4);
        }
    }
}
