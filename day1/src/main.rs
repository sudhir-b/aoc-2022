fn main() {
    // Read the contents of the file into a string
    let contents = include_str!("input.txt");

    // most efficient implementation is probably to just keep a top three and bump them off if any new totals are higher

    let mut all_calorie_counts = vec![];
    for substring in contents.split("\n\n") {
        let mut running_total: i32 = 0;
        for line in substring.lines() {
            let calorie_count: i32 = line.parse().expect("Error parsing string");
            running_total += calorie_count
        }
        all_calorie_counts.push(running_total)
    }

    all_calorie_counts.sort_unstable();

    let total: i32 = all_calorie_counts[all_calorie_counts.len() - 3..]
        .iter()
        .sum();

    println!("{:?}", total)
}
