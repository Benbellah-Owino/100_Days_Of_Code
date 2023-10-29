use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let age: u64 = args[1]
        .clone()
        .parse::<u64>()
        .expect("Please provide positive integer");

    println!("{}", calc_age(age));
}

fn calc_age(age: u64) -> u64 {
    age * 365
}
