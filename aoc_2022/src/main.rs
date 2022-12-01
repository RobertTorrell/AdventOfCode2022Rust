use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    println!("Hello, world!");
    day1();
}

fn day1(){
    let path = Path::new("./src/input_day1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s)
    };


    println!("---------------------------");

    let lines: Vec<&str> = s.split('\n').collect();
    let biggest_calories = 0;
    let biggest_calories_elf = 0;
    let current_calories = 0;

    for current_line in lines {
        println!("{}", current_line);
    }
}