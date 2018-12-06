use failure::Error;
use std::collections::HashSet;
use std::string::String;

type Result<T> = std::result::Result<T, Error>;

const FREQUENCY_CHANGES: &str = include_str!("day_1_input.txt");

pub fn run() -> Result<String> {
    let r1 = part1()?;
    let r2 = part2()?;
    Ok(format!("{}\n{}", r1, r2))
}

fn part2() -> Result<String> {
    Ok(format!(
        "Day 1, Part 2: {}",
        find_repeated_freq(FREQUENCY_CHANGES)?
    ))
}

fn find_repeated_freq<S: Into<String>>(input: S) -> Result<i32> {
    let changes = input.into();
    let mut freq = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(0);
    loop {
        for n in changes.clone().lines() {
            let n = n.trim();
            if n.len() == 0 {
                // skip blank lines
                continue;
            }
            let n: i32 = n.parse()?;
            freq += n;
            if frequencies.contains(&freq) {
                return Ok(freq);
            }
            frequencies.insert(freq);
        }
    }
}

fn part1() -> Result<String> {
    Ok(format!(
        "Day 1, Part 1: {}",
        freq_changes(FREQUENCY_CHANGES)?
    ))
}

fn freq_changes<S: Into<String>>(input: S) -> Result<i32> {
    let ins = input.into();
    ins.lines()
        .map(|s| s.trim())
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<i32>())
        .try_fold(0, |sum, val| -> Result<i32> { Ok(sum + val?) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn part1() {
        let t = freq_changes("1 \n2\n");
        assert_that!(t).is_ok().is_equal_to(3);
    }

    #[test]
    fn part2_1() {
        let t = find_repeated_freq("+1, -1".replace(", ", "\n"));
        assert_that!(t).is_ok().is_equal_to(0);
    }

    #[test]
    fn part2_2() {
        let t = find_repeated_freq("+3, +3, +4, -2, -4".replace(", ", "\n"));
        assert_that!(t).is_ok().is_equal_to(10);
    }

    #[test]
    fn part2_3() {
        let t = find_repeated_freq("-6, +3, +8, +5, -6".replace(", ", "\n"));
        assert_that!(t).is_ok().is_equal_to(5);
    }

    #[test]
    fn part2_4() {
        let t = find_repeated_freq("+7, +7, -2, -7, -4".replace(", ", "\n"));
        assert_that!(t).is_ok().is_equal_to(14);
    }
}
