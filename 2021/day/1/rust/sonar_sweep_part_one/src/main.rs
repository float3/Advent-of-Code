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

    let file_contents = fs::read_to_string(path).ok()?.replace("\r","");

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