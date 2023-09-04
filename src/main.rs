
use std::time::Instant;

/**
 * More doc types??
 */
fn main() {
    println!("Hello, world!");

    let x: i32 = 5;

    let y: i32 = if x == 5 {5} else {-1};
    println!("y = {y}");

    some_iterations();
    loop_lables();
    while_loop();
    array_loop();
}

/**
 * Run a loop for a billion iterations. Return None.
 */
fn some_iterations() {

    let now = Instant::now();
    let mut z: u64 = 0;

    let num_iterations: u64 = loop {
        z += 1;
        if z == 1_000 {
            let elapsed = now.elapsed().as_secs();
            println!("1 thousand!");
            println!("time elapsed = {elapsed} secs");
            break z;
        }        
    };

    println!("num iterations = {num_iterations}");
}

fn loop_lables() {
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            else if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1
    }

    println!("end count. count = {count}");
}

fn while_loop() {
    let mut countdown = 5;
    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("LIFTOFF!!");
}

fn array_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
//     while index < 5 {
//         println!("The value of a is {}", a[index]);
//         index += 1;
//     }

    // a better implementation of looping over values in an array
    for val in a {
        println!("the value is {val}");
    }
}
