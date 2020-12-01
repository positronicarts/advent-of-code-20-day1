use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TARGET: i32 = 2020;

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let numbers = get_numbers(opts.input);

    if opts.part == 1 {
        let result = seek_target(&numbers, TARGET);
        if let Some((a, b)) = result {
            println!("Product is {0}", a * b);
        } else {
            println!("Input error!!");
        }
    } else {
        let outer_list = numbers.clone();
        for num in outer_list {
            let result = seek_target(&numbers, TARGET - num);
            if let Some((a, b)) = result {
                println!("Product is {0}", num * a * b);
                break;
            }
        }
    }
}

fn seek_target(numbers: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut targets = Vec::<i32>::new();
    for num in numbers {
        let balance = target - num;
        if targets.contains(&num) {
            println!("{0} balances {1}!", num, balance);
            return Some((*num, balance));
        } else {
            println!("Pushing {0} from {1}", balance, num);
            targets.push(balance);
        }
    }
    None
}

fn get_numbers(filename: String) -> Vec<i32> {
    let mut numbers = Vec::<i32>::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(num_as_string) = line {
                if let Ok(num) = num_as_string.parse::<i32>() {
                    numbers.push(num);
                }
            }
        }
    }

    numbers
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
