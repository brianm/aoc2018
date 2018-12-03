use failure::Error;
use std::string::String;

type Result<T> = std::result::Result<T, Error>;

pub fn freq_changes<S: Into<String>>(input: S) -> i32 {
    let instr = input.into();
    let v: i32 = instr
        .split('\n')
        .map(|s| -> i32 { s.parse().unwrap() })
        .fold(0, |sum, val| -> i32 { sum + val});

    return v;
}

#[cfg(test)]
mod tests {

    use super::freq_changes;

    #[test]
    fn it_works() {
        let t = freq_changes("1\n2");
        assert_eq!(t, 3);
    }
}
