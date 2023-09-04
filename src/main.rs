
use std::time::Instant;

/**
 * More doc types??
 */
fn main() {
    println!("Hello, world!");

    let x: i32 = 5;

    let y: i32 = if x == 5 {5} else {-1};
    println!("y = {y}");

    billion_iterations();
}

/**
 * Run a loop for a billion iterations. Return None.
 */
fn billion_iterations() {

    let now = Instant::now();
    let mut z: u64 = 0;

    let num_iterations: u64 = loop {
        z += 1;
        if z == 1_000_000_000 {
            let elapsed = now.elapsed().as_secs();
            println!("1 billion!");
            println!("time elapsed = {elapsed} secs");
            break z;
        }        
    };

    println!("num iterations = {num_iterations}");
}
