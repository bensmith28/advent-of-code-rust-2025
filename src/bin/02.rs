use anyhow::*;
use std::fs::File;
use std::io::{read_to_string, BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[allow(unexpected_cfgs)]
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn shift_needed(n: usize) -> usize {
        let mut shift = 1;
        let mut temp = n;
        while temp > 0 {
            shift *= 10;
            temp /= 10;
        }
        return shift
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = read_to_string(reader)?;
        let mut sum: usize = 0;
        for range in input.split(',') {
            let mut split = range.split('-');
            let left = split.next().unwrap();
            let left_num: usize = left.trim().parse()?;
            let right = split.next().unwrap();
            let right_num: usize = right.trim().parse()?;

            let half_length = right.len() / 2 + right.len() % 2;
            let mut shift: usize = 1;
            for _ in 0..(right.len() - half_length) {
                shift *= 10;
            }
            let shift = shift;
            let min: usize = left_num / shift;
            let max: usize = right_num / shift; // is this the correct length?
            for test in min..=max {
                let testtest = test * shift_needed(test) + test;
                if testtest >= left_num && testtest <= right_num {
                    sum += testtest;
                }
            }
        }
        Ok(sum)
    }

    assert_eq!(1010, part1(BufReader::new("998-1012".as_bytes()))?);
    assert_eq!(99, part1(BufReader::new("95-115".as_bytes()))?);
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
