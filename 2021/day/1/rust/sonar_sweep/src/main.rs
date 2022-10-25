use std::fs;

const INPUT_FILE: &str = "../../inputs.txt";

fn main() {
    let increases = count_increases(INPUT_FILE);

    match increases {
        Some(count) => println!("The number of increases is: {}", count),
        None => println!("Error"),
    }

    let averaged_increases = count_averaged_increases(INPUT_FILE);

    match averaged_increases {
        Some(count) => println!("The number of averaged increases is: {}", count),
        None => println!("Error"),
    }
}

fn count_increases(path: &str) -> Option<i32> {
    let mut increased_counter = 0;

    let file_contents = fs::read_to_string(path).ok()?.replace("\r", "");

    let mut splits = file_contents.split("\n");
    let mut last_value = splits.next()?.parse::<i32>().ok()?;

    for current_line in splits {
        let current_value = str::parse::<i32>(current_line).ok()?;
        if current_value > last_value {
            increased_counter += 1;
        }
        last_value = current_value;
    }

    Some(increased_counter)
}

fn count_averaged_increases(path: &str) -> Option<i32> {
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
