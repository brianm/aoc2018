use failure::Error;
use std::string::String;

type Result<T> = std::result::Result<T, Error>;

pub fn freq_changes<S: Into<String>>(input: S) -> Result<i32> {
    let ins = input.into();
    ins
        .split('\n')
        .map(|s| s.trim())
        .map(|s| s.parse::<i32>())
        .try_fold(0, |sum, val| -> Result<i32> { Ok(sum + val?) })
}

#[cfg(test)]
mod tests {

    use super::freq_changes;

    #[test]
    fn it_works() {
        let t = freq_changes("1 \n2");
        assert_eq!(t, Ok(3));
    }
}
