use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut max_calorie = 0;
        let mut cur_calorie = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    cur_calorie = 0;
                    continue;
                }
                cur_calorie += ip.parse::<i32>().unwrap();
                if cur_calorie > max_calorie {
                    max_calorie = cur_calorie;
                }
                // println!("{ip}");
            }
        }
        println!("{max_calorie}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
