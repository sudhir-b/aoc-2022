use std::time::Instant;

//         [J]         [B]     [T]
//         [M] [L]     [Q] [L] [R]
//         [G] [Q]     [W] [S] [B] [L]
// [D]     [D] [T]     [M] [G] [V] [P]
// [T]     [N] [N] [N] [D] [J] [G] [N]
// [W] [H] [H] [S] [C] [N] [R] [W] [D]
// [N] [P] [P] [W] [H] [H] [B] [N] [G]
// [L] [C] [W] [C] [P] [T] [M] [Z] [W]
//  1   2   3   4   5   6   7   8   9

fn part_1() {
    let start = Instant::now();

    let mut state = vec![
        vec!["L", "N", "W", "T", "D"],
        vec!["C", "P", "H"],
        vec!["W", "P", "H", "N", "D", "G", "M", "J"],
        vec!["C", "W", "S", "N", "T", "Q", "L"],
        vec!["P", "H", "C", "N"],
        vec!["T", "H", "N", "D", "M", "W", "Q", "B"],
        vec!["M", "B", "R", "J", "G", "S", "L"],
        vec!["Z", "N", "W", "G", "V", "B", "R", "T"],
        vec!["W", "G", "D", "N", "P", "L"],
    ];

    let contents = include_str!("input.txt");

    for line in contents.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let number: usize = split_line[1].parse().unwrap();
        let source: usize = split_line[3].parse().unwrap();
        let destination: usize = split_line[5].parse().unwrap();

        let mut slice: Vec<&str> = vec![];
        for _ in 0..number {
            slice.push(state[source - 1].pop().unwrap());
        }

        state[destination - 1].append(&mut slice);
    }

    println!("{:?}", state);
    let tops: Vec<&str> = state.into_iter().map(|x| *x.last().unwrap()).collect();
    println!("{:?}", tops);

    let duration = start.elapsed();
    println!("{:?}", duration)
}

fn part_2() {
    let start = Instant::now();

    let mut state = vec![
        vec!["L", "N", "W", "T", "D"],
        vec!["C", "P", "H"],
        vec!["W", "P", "H", "N", "D", "G", "M", "J"],
        vec!["C", "W", "S", "N", "T", "Q", "L"],
        vec!["P", "H", "C", "N"],
        vec!["T", "H", "N", "D", "M", "W", "Q", "B"],
        vec!["M", "B", "R", "J", "G", "S", "L"],
        vec!["Z", "N", "W", "G", "V", "B", "R", "T"],
        vec!["W", "G", "D", "N", "P", "L"],
    ];

    let contents = include_str!("input.txt");
    for line in contents.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let number: usize = split_line[1].parse().unwrap();
        let source: usize = split_line[3].parse().unwrap();
        let destination: usize = split_line[5].parse().unwrap();

        let mut slice: Vec<&str> = vec![];
        for _ in 0..number {
            slice.push(state[source - 1].pop().unwrap());
        }

        slice.reverse();

        state[destination - 1].append(&mut slice);
    }

    println!("{:?}", state);
    let tops: Vec<&str> = state.into_iter().map(|x| *x.last().unwrap()).collect();
    println!("{:?}", tops);

    let duration = start.elapsed();
    println!("{:?}", duration)
}

fn main() {
    part_2()
}
