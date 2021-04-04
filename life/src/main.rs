extern crate rand;
// extern crate termion;

use std::{env, thread, time};
use std::fs::File;
use std::io::{BufReader, BufRead};

const MAX_SIZE: usize = 75;

fn main() {
    let mut world = [[0u8; MAX_SIZE]; MAX_SIZE];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..MAX_SIZE - 1 {
            for j in 0..MAX_SIZE - 1 {
                world[i][j] = if rand::random() { 1 } else { 0 };
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    println!("Population at generation {} is {}", generations, census(world));
    for _ in 0..100 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("");

        displayworld(world);
        println!("Population at generation {g} is {c}", g = generations, c = census(world));
        thread::sleep(time::Duration::from_secs(2));

        if census(world) == 0 {
            break;
        }
    }
}

fn populate_from_file(filename: String) -> [[u8; MAX_SIZE]; MAX_SIZE] {
    let mut newworld = [[0u8; MAX_SIZE]; MAX_SIZE];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let pairs: Vec<(usize, usize)> = reader.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut words = line.split_whitespace();
            let left = words.next().unwrap();
            let right = words.next().unwrap();
            (left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap())
        }).collect();

    for (x, y) in pairs {
        newworld[x][y] = 1;
    }

    newworld
}

fn displayworld(world: [[u8; MAX_SIZE]; MAX_SIZE]) {
    for i in 0..MAX_SIZE - 1 {
        for j in 0..MAX_SIZE - 1 {
            if world[i][j] == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn census(world: [[u8; MAX_SIZE]; MAX_SIZE]) -> usize {
    world
        .iter()
        .map(|row| row.iter()
            .filter(|&cell| *cell == 1)
            .count())
        .sum()
}

fn generation(world: [[u8; MAX_SIZE]; MAX_SIZE]) -> [[u8; MAX_SIZE]; MAX_SIZE] {
    let mut newworld = [[0u8; MAX_SIZE]; MAX_SIZE];

    for i in 0..MAX_SIZE - 1 {
        for j in 0..MAX_SIZE - 1 {
            let mut count = 0;
            if i > 0 {
                count += world[i - 1][j];
            }
            if i > 0 && j > 0 {
                count += world[i - 1][j - 1];
            }
            if i > 0 && j < MAX_SIZE - 1 {
                count += world[i - 1][j + 1];
            }
            if i < MAX_SIZE - 1 && j > 0 {
                count += world[i + 1][j - 1];
            }
            if i < MAX_SIZE - 1 {
                count += world[i + 1][j];
            }
            if i < MAX_SIZE - 1 && j < MAX_SIZE - 1 {
                count += world[i + 1][j + 1];
            }
            if j > 0 {
                count += world[i][j - 1];
            }
            if j < MAX_SIZE - 1 {
                count += world[i][j + 1];
            }

            if (count < 2) && (world[i][j] == 1) {
                newworld[i][j] = 0;
            }
            if world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            if world[i][j] == 0 && count == 3 {
                newworld[i][j] = 1;
            }
        }
    }

    newworld
}
