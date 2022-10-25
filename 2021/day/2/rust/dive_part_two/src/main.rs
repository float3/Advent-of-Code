use std::fs;

fn main() {
    let output = distance_traveled("../../inputs.txt");

    match output {
        Some(count) => println!("The Distance traveled is: {}", count),
        None => println!("Error"),
    }
}

fn distance_traveled(path: &str) -> Option<i32> {
    let file_contents = fs::read_to_string(path).ok()?.replace("\r", "");
    let splits = file_contents.split("\n");

    let mut depth = 0;
    let mut horizontal_pos = 0;
    let mut aim = 0;

    for instruction in splits {
        let mut instruction_splits = instruction.split(" ");

        let instruction = instruction_splits.next()?;

        let value = str::parse::<i32>(instruction_splits.next()?).ok()?;

        if instruction == "forward" {
            horizontal_pos += value;
            depth += value * aim;
        } else if instruction == "down" {
            aim += value;
        } else if instruction == "up" {
            aim -= value;
        }
        //match instruction {
        //    "forward" => horizontal_pos += value,
        //    "up" => aim -= value,
        //    "down" => aim += value,
        //    _ => println!("this shouldn't happen"),
        //}
    }

    Some(depth * horizontal_pos)
}
