use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let file = File::open("input.txt").expect("should be a path to text file");

    // create a buffered reader
    let reader = BufReader::new(file);

    let mut maximum_storage_capacity: u32 = 0;

    let mut total_calories: u32 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("expected a number");

        if line.is_empty() {
            if total_calories > maximum_storage_capacity {
                maximum_storage_capacity = total_calories;
            }
            total_calories = 0;
            continue;
        }

        total_calories += line.trim().parse::<u32>().expect("expected an integer");
    }

    println!("the elves storage is {}", maximum_storage_capacity);
}
