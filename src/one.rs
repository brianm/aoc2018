use failure::Error;
use std::string::String;

type Result<T> = std::result::Result<T, Error>;

pub fn part1() -> () {
    let input = include_str!("day_1_input.txt");
    println!("{}", freq_changes(input).unwrap());
}

pub fn freq_changes<S: Into<String>>(input: S) -> Result<i32> {
    let ins = input.into();
    ins.lines()
        .map(|s| s.trim())
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<i32>())
        .try_fold(0, |sum, val| -> Result<i32> { Ok(sum + val?) })
}

#[cfg(test)]
mod tests {
    use super::freq_changes;
    use spectral::prelude::*;

    #[test]
    fn it_works() {
        let t = freq_changes("1 \n2\n");
        assert_that!(t).is_ok().is_equal_to(3);
    }
}
