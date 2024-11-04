use std::io::{stdin, Read};

use algorithms::load_cryptography;

mod algorithms;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let algorithm_binding = args.first();

    if algorithm_binding.is_none() {
        println!("Missing algorithm name.");
        return;
    }

    let algorithm_name = algorithm_binding.unwrap();

    let mut input = String::new();

    stdin()
    .read_to_string(&mut input)
    .expect("The input could not be read.");

    let enconded = load_cryptography(algorithm_name.clone(), input);

    if enconded.is_none() {
        println!("Could not find the \"{algorithm_name}\" algorithm.");
        return;
    }

    println!("{}", enconded.unwrap());
}