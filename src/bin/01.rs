use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

const TEST1: &str = "\
L50
R10
";

#[allow(unexpected_cfgs)]

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter: isize = 50;
        let mut zeros = 0usize;
        for l in reader.lines() {
            let line = l?;
            let dir = &line[0..1];
            let mag: isize = line[1..].parse()?;
            match dir {
                "R" => counter += mag,
                "L" => counter -= mag,
                _ => return Err(anyhow!("Invalid direction!")),
            }
            while counter > 99 {
                counter -= 100;
            }
            while counter < 0 {
                counter += 100;
            }
            if counter == 0 {
                zeros += 1;
            }
        }
        Ok(zeros)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter: isize = 50;
        let mut zeros: usize = 0;
        for l in reader.lines() {
            let line = l?;
            let dir = &line[0..1];
            let mag: isize = line[1..].trim().parse()?;
            match dir {
                "R" => {
                    let next = counter + mag;
                    //println!("Right, counter = {}, mag = {}, next = {}", counter, mag, next);
                    //println!("{} is greater than zero, add {}, resolve to {}", next, next / 100, next % 100);
                    zeros += (next / 100) as usize;
                    counter = next % 100;
                },
                "L" => {
                    let next = counter - mag;
                    //println!("Left, counter = {}, mag = {}, next = {}", counter, mag, next);
                    if next < 0 && next > -100 && counter == 0 {
                        //println!("was on zero, no add");
                        counter = (next + 100) % 100;
                        println!("A - counter: {}, next: {}", counter, next);
                    } else if next < 0 && counter == 0 {
                        // was on zero, multiple times around
                        let m = (next * -1) / 100;
                        //counter = (next + 100 * m) % 100;
                        counter = next % 100 + 100;
                        println!("B - counter: {}, next: {}", counter, next);
                        zeros += m as usize;
                    } else if next < 0 {
                        //println!("{} is less than zero, add {}, resolve to {}", next, next * -1 / 100 + 1, next % 100 + 100);
                        zeros += (next * -1 / 100 + 1) as usize;
                        counter = (next % 100 + 100) % 100;
                        println!("C - counter: {}, next: {}", counter, next);
                    } else if next == 0 {
                        //println!("Entered zeros, add 1");
                        zeros += 1;
                        counter = next;
                        println!("D - counter: {}, next: {}", counter, next);
                    } else {
                        counter = next;
                        println!("E - counter: {}, next: {}", counter, next);
                    }
                },
                _ => return Err(anyhow!("Invalid direction!")),
            }
            //println!("zeros = {}", zeros);
        }
        Ok(zeros)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(1, part2(BufReader::new(TEST1.as_bytes()))?);
    // land on zero
    assert_eq!(1, part2(BufReader::new("L50".as_bytes()))?);
    // land on zero, exit left
    assert_eq!(1, part2(BufReader::new("L50\nL1".as_bytes()))?);
    // land on zero, exit right
    assert_eq!(1, part2(BufReader::new("L50\nR1".as_bytes()))?);
    // past zero, going left
    assert_eq!(1, part2(BufReader::new("L55".as_bytes()))?);
    // past zero, going right
    assert_eq!(1, part2(BufReader::new("R55".as_bytes()))?);
    // land on 100
    assert_eq!(1, part2(BufReader::new("R50".as_bytes()))?);
    // land on 100, exit right
    assert_eq!(1, part2(BufReader::new("R50\nR1".as_bytes()))?);
    // land on 100, exit left
    assert_eq!(1, part2(BufReader::new("R50\nL1".as_bytes()))?);
    // land on zero, twice around
    assert_eq!(2, part2(BufReader::new("L150".as_bytes()))?);
    // past zero, twice around
    assert_eq!(2, part2(BufReader::new("L151".as_bytes()))?);
    // land on 100, twice around
    assert_eq!(2, part2(BufReader::new("R150".as_bytes()))?);
    // past 100, twice around
    assert_eq!(2, part2(BufReader::new("R151".as_bytes()))?);
    // start from zero, left multiples
    assert_eq!(2, part2(BufReader::new("L50\nL120".as_bytes()))?);


    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // 6580 is too high, 6630, 6609, 6558
    //endregion

    Ok(())
}

