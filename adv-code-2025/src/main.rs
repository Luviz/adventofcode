pub mod day_01;

use std::{collections::HashMap, env};

pub trait Runnable {
    fn run(&self);
}

fn main() {
    let days = load_days();

    let args: Vec<String> = env::args().collect();
    println!("Advent of code 2025");
    let day_numb = args.get(1);

    let runner = match day_numb {
        None => {
            println!("you need a day number from 01 to 31");
            panic!("day number not define");
        }
        Some(dn) => {
            match days.get(dn.as_str()) {
                None => {
                    println!("the give day {}, dose not exist.", dn);
                    panic!("implementation dose not exist");
                }
                Some(r) => r,
            }
        }
    };

    runner.run();
    println!("DONE!")
}

fn load_days() -> HashMap<&'static str, Box<dyn Runnable>> {
    let mut map: HashMap<&str, Box<dyn Runnable>> = HashMap::new();
    map.insert("01", day_01::get_task());

    map
}
