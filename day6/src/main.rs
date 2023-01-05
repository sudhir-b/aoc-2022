use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn part_1() {
    let start = Instant::now();

    let file_name = "input.txt";

    // Read the contents of the file into a string
    let contents = fs::read_to_string(file_name).expect("Error reading file");

    let mut window: Vec<char> = vec![];

    for i in 0..4 {
        let next = contents.chars().nth(i).unwrap();
        window.push(next);
    }

    for i in 4..contents.len() {
        let mut uniques: HashSet<char> = HashSet::new();
        uniques.extend(window.iter());

        if uniques.len() == 4 {
            println!("{}", i);
            break;
        } else {
            let next = contents.chars().nth(i).unwrap();
            window.remove(0);
            window.push(next);
        }
    }

    let duration = start.elapsed();
    println!("{:?}", duration)
}

fn part_2() {
    let start = Instant::now();

    let file_name = "input.txt";

    // Read the contents of the file into a string
    let contents = fs::read_to_string(file_name).expect("Error reading file");

    let mut window: Vec<char> = vec![];

    for i in 0..14 {
        let next = contents.chars().nth(i).unwrap();
        window.push(next);
    }

    for i in 14..contents.len() {
        let mut uniques: HashSet<char> = HashSet::new();
        uniques.extend(window.iter());

        if uniques.len() == 14 {
            println!("{}", i);
            break;
        } else {
            let next = contents.chars().nth(i).unwrap();
            window.remove(0);
            window.push(next);
        }
    }

    let duration = start.elapsed();
    println!("{:?}", duration)
}

fn main() {
    part_2()
}
