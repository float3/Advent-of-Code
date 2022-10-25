use std::fs;

fn main() {
    let output = count_increases("../../inputs.txt");

    match output {
        Some(count) => println!("The number of increases is: {}", count),
        None => println!("Error"),
    }
}

fn count_increases(path: &str) -> Option<i32> {
    let mut increased_counter = 0;

    let file_contents = fs::read_to_string(path).ok()?.replace("\r", "");

    let splits = file_contents
        .split("\n")
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .ok()?;

    let mut windows = splits.windows(3);

    let first = windows.next()?;

    let mut last_sum: i32 = first.iter().sum();

    for window in windows {
        let current_sum = window.iter().sum();
        if current_sum > last_sum {
            increased_counter += 1;
        }
        last_sum = current_sum;
    }

    Some(increased_counter)
}
