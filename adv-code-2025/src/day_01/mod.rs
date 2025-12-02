use std::fs;

use crate::Runnable;

pub struct One {
    sample_file: String,
    file: String,
}

impl One {
    fn _run(&self, path: &String) -> i32 {
        let mut curr_val = 50;
        let mut zero_count = 0;
        let mut zero_cross = 0;
        let contents = fs::read_to_string(path).expect("file not found");
        for ln in contents.lines() {
            let oc: i32;
            let (t, value) = parse(ln);
            // println!("{}>>{}", t, value);
            match t {
                "R" => (curr_val, oc) = right(value, curr_val),
                "L" => (curr_val, oc) = left(value, curr_val),
                _ => panic!("wops!"),
            }
            // println!("curr {}, val: {}", curr_val, oc);
            zero_cross += oc;
            if curr_val == 0 {
                zero_count += 1;
            }
        }
        println!("p1 {}, p2 {}", zero_count, zero_cross);
        zero_count
    }

    fn run_sample(&self) {
        println!("Day 01, running sample");
        println!("loading {}", self.sample_file);
        let z = self._run(&self.sample_file);
        println!("sample result '{}'", z);
    }
    fn run_pt1(&self) {
        println!("Day 01, part one");
        println!("loading {}", self.file);
        let z = self._run(&self.file);
        println!("pt1 result '{}'", z);
    }
}

impl Runnable for One {
    fn run(&self) {
        self.run_sample();
        self.run_pt1();
    }
}

fn left(val: i32, curr: i32) -> (i32, i32) {
    turn(-val, curr)
}
fn right(val: i32, curr: i32) -> (i32, i32) {
    turn(val, curr)
}

fn turn(val: i32, curr: i32) -> (i32, i32) {
    let delta = curr + val;
    let pos = delta.rem_euclid(100);
    // let rot = delta.div_euclid(100).abs();
    let rot = (curr + val).div_euclid(100).abs() - (curr.div_euclid(100).abs());
    (pos, rot)
}

fn parse(s: &str) -> (&str, i32) {
    let (f, r) = s.split_at(1);
    let val: i32 = r.parse::<i32>().unwrap();
    (f, val)
}

pub fn get_task() -> Box<dyn Runnable> {
    Box::new(One {
        sample_file: "./resources/01/sample".to_string(),
        file: "./resources/01/part1".to_string(),
    })
}
