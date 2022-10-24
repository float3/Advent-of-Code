use std::fs;

fn main() {
    let mut increased_counter = 0;

    let file_contents = fs::read_to_string("../../../inputs.txt").unwrap().replace("\r","");

    let mut splits = file_contents.split("\n");
    let mut last_value = splits.next().unwrap().parse::<i32>().unwrap();

    for current_line in splits {
        let current_value = str::parse::<i32>(current_line).unwrap();
        if current_value > last_value {
            increased_counter += 1;
        }
        last_value = current_value;
    }

    println!("{increased_counter}");
}
