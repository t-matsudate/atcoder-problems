use std::{
    io::{
        Result as IOResult,
        stdin
    },
    str::{
        Chars
    }
};

pub fn main() -> IOResult<()> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let input_trimmed = input.trim_end();

    println!("{}", problem_a(input_trimmed));
    Ok(())
}

pub fn problem_a(bits: &str) -> usize {
    bits.chars().fold(
        0,
        |count, bit| {
            if bit == '1' {
                count + 1
            } else {
                count
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use std::time::{
        SystemTime,
        SystemTimeError
    };
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_a(bits: String) -> TestResult {
        if bits.len() != 3 {
            TestResult::discard()
        } else if let Some(_) = bits.find(|c| c != '0' || c != '1') {
            TestResult::discard()
        } else {
            let mut _bits = bits.clone();

            _bits.retain(|c| c == '0');

            let count_ones = bits.len();

            if count_ones == problem_a(bits.as_str()) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[test]
    fn test_execution_time() -> Result<(), SystemTimeError> {
        let time_start = SystemTime::now();

        problem_a(&"000");

        let time_end = SystemTime::now();
        let time_diff = time_end.duration_since(time_start)?;

        assert!(time_diff.as_secs() <= 2, "The code of ABC081A exceeded the time limit!");
        Ok(())
    }
}
