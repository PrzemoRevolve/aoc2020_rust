use std::collections::HashMap;

use crate::{first_answer, read_input_no_skip, second_answer};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Coords {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Size {
    xy: isize,
    z: isize,
    w: isize,
}

type Map = HashMap<Coords, State>;

pub fn run(day: u8) {
    let input = read_input_no_skip(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let (mut map, mut size) = parse(input);
    print_map(&map, &size);

    for _ in 0..6 {
        let res = run_cycle_3d(map, size);
        map = res.0;
        size = res.1;

        // print_map(&map, &size);
    }

    let ans = map.iter().filter(|s| *s.1 == State::Active).count();

    String::from(format!("{:?}", ans))
}

fn second_problem(input: &Vec<String>) -> String {
    let (mut map, mut size) = parse(input);
    // print_map(&map, &size);

    for i in 0..6 {
        let res = run_cycle_4d(map, size);
        map = res.0;
        size = res.1;
        println!("Ran cycle {}", i);
        // print_map(&map, &size);
    }

    let ans = map.iter().filter(|s| *s.1 == State::Active).count();

    String::from(format!("{:?}", ans))
}

fn parse(input: &Vec<String>) -> (Map, Size) {
    let mut map: Map = HashMap::new();
    let width = input[0].len() as isize / 2;

    for (y, l) in input.iter().enumerate() {
        for (x, s) in l.chars().enumerate() {
            let state = match s {
                '#' => State::Active,
                _ => State::Inactive,
            };

            map.insert(
                Coords {
                    x: x as isize - width,
                    y: y as isize - width,
                    z: 0,
                    w: 0,
                },
                state,
            );
        }
    }

    (
        map,
        Size {
            z: 1,
            xy: width,
            w: 1,
        },
    )
}

fn find_near(map: &Map, center: &Coords) -> Map {
    let mut near: Map = HashMap::new();

    for w in (center.w - 1)..=(center.w + 1) {
        for z in (center.z - 1)..=(center.z + 1) {
            for y in (center.y - 1)..=(center.y + 1) {
                for x in (center.x - 1)..=(center.x + 1) {
                    let coords = Coords { x, y, z, w };

                    match map.get(&coords) {
                        Some(state) => near.insert(coords, *state),
                        None => near.insert(coords, State::Inactive),
                    };
                }
            }
        }
    }

    near
}

fn run_cycle_3d(map: Map, size: Size) -> (Map, Size) {
    let mut new_map: Map = HashMap::new();
    let new_size = Size {
        z: size.z + 1,
        xy: size.xy + 1,
        w: size.w,
    };

    for z in (-new_size.z)..=(new_size.z) {
        for y in (-new_size.xy)..=new_size.xy {
            for x in (-new_size.xy)..=new_size.xy {
                let coords = Coords { x, y, z, w: 0 };
                let near = find_near(&map, &coords);
                let curr_state = map.get(&coords).unwrap_or(&State::Inactive);
                let actives: isize = near
                    .iter()
                    .map(|(_, s)| match s {
                        State::Active => 1,
                        State::Inactive => 0,
                    })
                    .sum();
                let state;
                if ((actives == 3 || actives == 4) && curr_state == &State::Active)
                    || (actives == 3 && curr_state == &State::Inactive)
                {
                    state = State::Active;
                } else {
                    state = State::Inactive;
                }
                // println!("{:?} {:?} {:?} {:?}", coords, curr_state, actives, state);

                new_map.insert(coords, state);
            }
        }
    }

    (new_map, new_size)
}
fn run_cycle_4d(map: Map, size: Size) -> (Map, Size) {
    let mut new_map: Map = HashMap::new();
    let new_size = Size {
        z: size.z + 1,
        xy: size.xy + 1,
        w: size.w + 1,
    };

    for w in (-new_size.w)..=(new_size.w) {
        for z in (-new_size.z)..=(new_size.z) {
            for y in (-new_size.xy)..=new_size.xy {
                for x in (-new_size.xy)..=new_size.xy {
                    let coords = Coords { x, y, z, w };
                    let near = find_near(&map, &coords);
                    let curr_state = map.get(&coords).unwrap_or(&State::Inactive);
                    let actives: isize = near
                        .iter()
                        .map(|(_, s)| match s {
                            State::Active => 1,
                            State::Inactive => 0,
                        })
                        .sum();
                    let state;
                    if ((actives == 3 || actives == 4) && curr_state == &State::Active)
                        || (actives == 3 && curr_state == &State::Inactive)
                    {
                        state = State::Active;
                    } else {
                        state = State::Inactive;
                    }
                    // println!("{:?} {:?} {:?} {:?}", coords, curr_state, actives, state);

                    new_map.insert(coords, state);
                }
            }
        }
    }

    (new_map, new_size)
}

fn print_map(map: &Map, size: &Size) -> () {
    for z in (-size.z)..=(size.z) {
        println!("Z: {}", z);

        for y in (-size.xy)..=size.xy {
            for x in (-size.xy)..=size.xy {
                let coords = Coords { x, y, z, w: 0 };
                let state = map.get(&coords);
                let c = match state {
                    Some(State::Active) => '#',
                    _ => '.',
                };

                print!("{}", c);
            }

            print!("\n");
        }
    }

    print!("\n");
}
