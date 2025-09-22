#![allow(unused)]
// Exercise: Fix the code to make it compile and pass the assertions

// Constants live inside the compiled code while variables (let) live in memory:
const NUM: u32 = 123;

fn main() {
    // Exercise 1: Make this variable mutable
    let mut count = 1;
    count += 1;

    // Inline variable:
    println!("count: {count}");

    // Same code (above uses type inference):
    let mut y: i32 = 1;
    y += 1;

    // Shadowing (redeclare variables):
    let x: bool = true;

    // Type placeholder (explicitly tell rust to figure out the type):
    let y: _ = 2.3;

    // Variable as argument:
    println!("{}", y);

    // Positional arguments for vars:
    println!("{0} + {0} = {1}", y, y + y);

    // Debug feature (# added for multiline / pretty print):
    println!("Debug = {:#?}", x);

}
