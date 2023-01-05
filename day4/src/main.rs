use std::time::Instant;

fn part_1() {
    let start = Instant::now();
    let contents = include_str!("input.txt");

    // This can't be the best way of doing this in a functional style surely!
    let ranges: Vec<Vec<Vec<usize>>> = contents
        .lines()
        .map(|x| {
            x.split(",")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.split("-").map(|x| x.parse::<usize>().unwrap()).collect())
                .collect::<Vec<Vec<usize>>>()
        })
        .filter(|x| {
            let first_comp = x[0][0] <= x[1][0] && x[0][1] >= x[1][1];
            let second_comp = x[0][0] >= x[1][0] && x[0][1] <= x[1][1];
            return first_comp || second_comp;
        })
        .collect();

    println!("{:?}", ranges.len());

    let duration = start.elapsed();
    println!("{:?}", duration)
}

fn part_2() {
    let start = Instant::now();
    let contents = include_str!("input.txt");

    // This can't be the best way of doing this in a functional style surely!
    let ranges: Vec<Vec<Vec<usize>>> = contents
        .lines()
        .map(|x| {
            x.split(",")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.split("-").map(|x| x.parse::<usize>().unwrap()).collect())
                .collect::<Vec<Vec<usize>>>()
        })
        .filter(|x| {
            let overlap_1 = x[1][0] <= x[0][0] && x[0][0] <= x[1][1];
            let overlap_2 = x[1][0] <= x[0][1] && x[0][1] <= x[1][1];
            let overlap_3 = x[0][0] <= x[1][0] && x[1][0] <= x[0][1];
            let overlap_4 = x[0][0] <= x[1][1] && x[1][1] <= x[0][1];
            return overlap_1 || overlap_2 || overlap_3 || overlap_4;
        })
        .collect();

    println!("{:?}", ranges.len());

    let duration = start.elapsed();
    println!("{:?}", duration)
}

fn main() {
    part_2()
}
