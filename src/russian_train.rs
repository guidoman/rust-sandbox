extern crate itertools;

use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Identity {
    label_id: String,
    city_name: String,
}

impl Display for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            r#"Character with label "{}" comes from {}"#,
            self.label_id, self.city_name
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PassengerLabel {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug, PartialEq)]
enum City {
    Moscow,
    Leningrad,
    Tula,
    Kiev,
    Kharkov,
    Odessa,
}

#[derive(Debug)]
struct DiversityConstraint {
    lhs: PassengerLabel,
    rhs: City,
}

fn main() {
    let constraints = vec![
        DiversityConstraint { // 1
            lhs: PassengerLabel::A,
            rhs: City::Moscow,
        },
        DiversityConstraint { // 1 + 2
            lhs: PassengerLabel::A,
            rhs: City::Leningrad,
        },
        DiversityConstraint { // 1 + 3
            lhs: PassengerLabel::A,
            rhs: City::Tula,
        },
        DiversityConstraint { // 2
            lhs: PassengerLabel::E,
            rhs: City::Leningrad,
        },
        DiversityConstraint { // 2 + 1
            lhs: PassengerLabel::E,
            rhs: City::Moscow,
        },
        DiversityConstraint { // 2 + 3
            lhs: PassengerLabel::E,
            rhs: City::Tula,
        },
        DiversityConstraint { // 3
            lhs: PassengerLabel::C,
            rhs: City::Tula,
        },
        DiversityConstraint { // 3 + 1
            lhs: PassengerLabel::C,
            rhs: City::Moscow,
        },
        DiversityConstraint { // 3 + 2
            lhs: PassengerLabel::C,
            rhs: City::Leningrad,
        },
        DiversityConstraint { // 4
            lhs: PassengerLabel::B,
            rhs: City::Tula,
        },
        DiversityConstraint { // 4
            lhs: PassengerLabel::F,
            rhs: City::Tula,
        },
        DiversityConstraint { // 5
            lhs: PassengerLabel::A,
            rhs: City::Kharkov,
        },
        DiversityConstraint { // 6
            lhs: PassengerLabel::C,
            rhs: City::Odessa,
        },
        DiversityConstraint { // 7
            lhs: PassengerLabel::B,
            rhs: City::Moscow,
        },
        DiversityConstraint { // 7 + 8
            lhs: PassengerLabel::B,
            rhs: City::Kharkov,
        },
        DiversityConstraint { // 8
            lhs: PassengerLabel::C,
            rhs: City::Kharkov,
        },
        DiversityConstraint { // 8 + 7
            lhs: PassengerLabel::C,
            rhs: City::Moscow,
        },
    ];

    let passengers_labels = vec![
        PassengerLabel::A,
        PassengerLabel::B,
        PassengerLabel::C,
        PassengerLabel::D,
        PassengerLabel::E,
        PassengerLabel::F,
    ];
    let passengers_origins = vec![
        City::Moscow,
        City::Leningrad,
        City::Tula,
        City::Kiev,
        City::Kharkov,
        City::Odessa,
    ];
    let mut solutions = Vec::new();
    let mut constraints_counters : Vec<u8>= vec![0; constraints.len()];

    for perm in passengers_labels
        .iter()
        .permutations(passengers_labels.len())
        .unique()
    {
        let check_result = check_solution(&perm, &passengers_origins, &constraints, &mut constraints_counters);
        if check_result {
            solutions.push(perm);
        }
    }

    if solutions.len() == 0 {
        println!("No solution found ");
    } else {
        for solution in solutions {
            println!("Found solution!");
            for i in 0..solution.len() {
                println!("{:?} = {:?}", solution[i], passengers_origins[i]);
            }
        }
        println!("Contraints counters:");
        println!("{:?}", constraints_counters);
    }
}

fn check_solution(
    labels: &Vec<&PassengerLabel>,
    origins: &Vec<City>,
    constraints: &Vec<DiversityConstraint>,
    constraints_counters: &mut Vec<u8>
) -> bool {
    assert!(origins.len() == labels.len());
    assert!(constraints.len() == constraints_counters.len());
    for i in 0..labels.len() {
        let label = labels[i];
        let origin = &origins[i];
        for (i, constraint) in constraints.iter().enumerate() {
            if label == &constraint.lhs && origin == &constraint.rhs {
                constraints_counters[i] += 1;
                return false;
            }
        }
    }
    return true;
}
