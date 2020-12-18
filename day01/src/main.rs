use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
	let mut numbers = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
    	for line in lines {
    		if let Ok(ip) = line {
    			let l = ip.parse::<i32>().unwrap();
    			numbers.push(l)
    		}
    	}
    }

    part1(&mut numbers);

    part2(&mut numbers);
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn part1(xs: &mut Vec<i32>) {
    xs.sort();

    let mut low = 0;
    let mut high = xs.len() - 1;

    while low < high {
        let l = xs[low];
        let h = xs[high];
        let sum = l + h;
        if sum == 2020 {
            println!("Part 1: {:?} * {:?} = {:?}", l, h, l * h);
            break;
        }
        if sum > 2020 {
            high -= 1;
        }
        if sum < 2020 {
            low += 1;
        }
    }
}

fn part2(xs: &mut Vec<i32>) {
    xs.sort();



    for (i, x) in xs.iter().enumerate() {
        let target = 2020 - x;
        let mut low = 0;
        let mut high = xs.len() - 1;
        while low < high {
            if i == low {
                low += 1;
                continue;
            }
            if i == high {
                high -= 1;
                continue;
            }
            let l = xs[low];
            let h = xs[high];
            let sum = l + h;

            if sum == target {
                println!("Part 2: {:?} * {:?} * {:?} = {:?}", l, h, x, l * h * x);
                break;
            }
            if sum > target {
                high -= 1;
            }
            if sum < target {
                low += 1;
            }
        }   
    }
}