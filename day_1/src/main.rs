use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> Result<(), io::Error> {
    let file = File::open("./day_1/input.txt")?;
    let reader = BufReader::new(file);
    let numbers = reader.lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|l| l.trim().parse().expect(&format!("{} wasn't a valid u64", l)))
        .collect::<Vec<u64>>();
    
    for i in 0..(numbers.len() - 2) {
        for j in (i+1)..(numbers.len() - 1) {
            for k in (j+1)..numbers.len() {
                let x = numbers[i];
                let y = numbers[j];
                let z = numbers[k];
                if x + y + z == 2020 {
                    println!("{} + {} + {} = 2020, {} * {} * {} = {}", x, y, z, x, y, z, x * y * z);
                }
            }
        }
    }
    
    Ok(())
}
