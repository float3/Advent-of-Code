use std::fs;

const INPUT_FILE: &str = "../../inputs.txt";

fn main() {
    let power_consumption = power_consumption(INPUT_FILE);

    match power_consumption {
        Some(count) => println!("The distance traveled is: {}", count),
        None => println!("Error"),
    }

}

fn power_consumption(path: &str) -> Option<i32> {
    let file_contents = fs::read_to_string(path).ok()?.replace("\r", "");
    let splits = file_contents.split("\n");

    let gamma = 0;
    let epsilon = 0;

    for split in splits {

    }

    Some(2)
}
