use std::{
    io::{
        Result as IOResult,
        stdin
    }
};

pub fn main() -> IOResult<()> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let input_trimmed = input.trim_end();

    println!("{}", problem_a(input_trimmed));
    Ok(())
}

pub fn problem_a(bits: &str) -> u32 {
    bits.chars().fold(
        0u32,
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
    use std::time::SystemTime;
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_a(bits: String) -> TestResult {
        if bits.len() != 3 {
            TestResult::discard()
        } else if let Some(_) = bits.find(|c| c != '0' || c != '1') {
            TestResult::discard()
        } else {
            let ones = i8::from_str_radix(bits.as_str(), 2).unwrap().count_ones();

            if ones == problem_a(bits.as_str()) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(bits: String) -> TestResult {
        if bits.len() != 3 {
            TestResult::discard()
        } else if let Some(_) = bits.find(|c| c != '0' || c != '1') {
            TestResult::discard()
        } else {
            let time_start = SystemTime::now();

            problem_a(bits.as_str());

            let time_end = SystemTime::now();
            let time_diff = time_end.duration_since(time_start).unwrap();

            if time_diff.as_secs() <= 2 {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }
}
