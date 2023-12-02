use std::path;

/**
 * --- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look.
The Elves have even given you a map; on it, they've used stars to mark the top fifty
locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations,
you need to check all fifty stars by December 25th.

Collect stars by solving puzzles.
Two puzzles will be made available on each day in the Advent calendar;
the second puzzle is unlocked when you complete the first.
Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough")
and where they're even sending you ("the sky") and why your map
looks mostly blank ("you sure ask a lot of questions")
and hang on did you just say the sky ("of course, where do you think snow comes from")
when you realize that the Elves are already loading you into a trebuchet
("please hold still, we need to strap you in").

As they're making the final adjustments,
they discover that their calibration document (your puzzle input)
has been amended by a very young Elf who was apparently just excited to show off her art skills.
Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text;
each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit
and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77.
Adding these together produces 142.

Consider your entire calibration document.
What is the sum of all of the calibration values?
 */

fn main() {
    let file =
        std::fs::read_to_string(path::Path::new("day1/input.txt")).expect("Unable to read file");
    let lines: Vec<&str> = file.lines().collect();
    let sum = solve_puzzle(lines);
    println!("The sum of all calibration values is {}", sum);
}

fn get_numbers_from_lines(lines: Vec<&str>) -> Vec<u32> {
    lines
        .into_iter()
        .map(|line| {
            let mut first_digit = 0;
            let mut last_digit = 0;

            line.chars().for_each(|c| {
                if c.is_digit(10) {
                    if first_digit == 0 {
                        // unwrap is fine because we know c is a digit
                        first_digit = c.to_digit(10).unwrap();
                    }
                    last_digit = c.to_digit(10).unwrap();
                }
            });
            first_digit * 10 + last_digit
        })
        .collect()
}

fn solve_puzzle(lines: Vec<&str>) -> u32 {
    let numbers = get_numbers_from_lines(lines);
    println!("Numbers: {:?}", numbers);
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_from_lines() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let numbers = get_numbers_from_lines(lines);
        assert_eq!(numbers, vec![12, 38, 15, 77]);
    }

    #[test]
    fn test_solve_puzzle() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let sum = solve_puzzle(lines);
        assert_eq!(sum, 142);
    }
}
