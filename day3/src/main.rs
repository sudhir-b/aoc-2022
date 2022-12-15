use std::collections::HashMap;
use std::fs;

fn main() {
    let file_name = "input.txt";

    // Read the contents of the file into a string
    let contents = fs::read_to_string(file_name).expect("Error reading file");

    let alphabet = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .enumerate()
        .map(|(i, c)| (c as char, i as u8 + 1))
        .collect::<Vec<(char, u8)>>();

    let priorities: HashMap<char, u8> = HashMap::from_iter(alphabet);

    let mut priority_sum = 0usize;

    for line in contents.lines().collect::<Vec<&str>>().chunks(3) {
        let mut first_bitset = [0u8; 52];
        let mut second_bitset = [0u8; 52];
        let mut third_bitset = [0u8; 52];

        for thing in line[0].chars() {
            let char_priority = priorities.get(&thing).unwrap();
            first_bitset[*char_priority as usize - 1] = 1;
        }

        for thing in line[1].chars() {
            let char_priority = priorities.get(&thing).unwrap();
            second_bitset[*char_priority as usize - 1] = 1;
        }

        for thing in line[2].chars() {
            let char_priority = priorities.get(&thing).unwrap();
            third_bitset[*char_priority as usize - 1] = 1;
        }

        let bitwise_and: Vec<usize> = first_bitset
            .into_iter()
            .zip(second_bitset.into_iter())
            .zip(third_bitset.into_iter())
            .map(|((a, b), c)| a & b & c)
            .enumerate()
            .filter(|(_, x)| *x == 1)
            .map(|(i, _)| i + 1)
            .collect();

        priority_sum += bitwise_and.first().unwrap();
    }

    println!("{:?}", priority_sum)
}
