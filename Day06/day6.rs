use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("{}", simulate(80));
    println!("{}", simulate(256)); 
}

fn simulate(days: i16) -> u128 {
    let mut fish = parse_input();
    for _ in 0..days {
        fish[7] += fish[0];
        fish.rotate_left(1);
    }
    return fish.iter().sum();
}

fn parse_input() -> [u128; 9]  {
    let file = File::open("./input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let mut fish = [0;9];
    reader.lines().for_each(|line| {
        line.unwrap().split(',').map(|s| s.parse().unwrap()).collect::<Vec<i32>>().iter().for_each(|i| {
            fish[*i as usize] += 1;
        });
    });
    return fish;
}
