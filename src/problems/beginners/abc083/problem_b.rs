use std::{
    io::{
        Result as IOResult,
        stdin
    },
    str::{
        FromStr
    }
};

pub fn main() -> IOResult<()> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let input_trimmed = input.trim_end();
    let mut input_split = input_trimmed.split(" ");
    let str_n = input_split.next().expect("'N' couldn't be read!");
    let str_a = input_split.next().expect("'A' couldn't be read!");
    let str_b = input_split.next().expect("'B' couldn't be read!");
    let n = u32::from_str(str_n).expect("'N' isn't a number!");
    let a = u32::from_str(str_a).expect("'A' isn't a number!");
    let b = u32::from_str(str_b).expect("'B' isn't a number!");

    println!("{}", problem_b(n, a, b));
    Ok(())
}

pub fn problem_b(n: u32, a: u32, b: u32) -> u32 {
    let mut sum_satisfied: u32 = 0;

    for i in 0..(n + 1) {
        let mut sum_digit: u32 = 0;

        for c in format!("{}", i).chars() {
            sum_digit += c.to_digit(10).unwrap();
        }

        if a <= sum_digit && sum_digit <= b {
            sum_satisfied += i;
        }
    }

    sum_satisfied
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_b(n: u32, a: u32, b: u32) -> TestResult {
        if n < 1 || n > 10000 {
            TestResult::discard()
        } else if (a < 1 || a > 36) || (b < 1 || b > 36) || (a > b) {
            TestResult::discard()
        } else {
            let mut sum_satisfied: u32 = 0;

            for i in 1..(n + 1) {
                let mut sum_digit: u32 = 0;

                for c in format!("{}", i).chars() {
                    sum_digit += c.to_digit(10).unwrap();
                }

                if a <= sum_digit && sum_digit <= b {
                    sum_satisfied += i;
                }
            }

            if sum_satisfied == problem_b(n, a, b) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(n: u32, a: u32, b: u32) -> TestResult {
        if n < 1 || n > 10000 {
            TestResult::discard()
        } else if (a < 1 || a > 36) || (b < 1 || b > 36) || (a > b) {
            TestResult::discard()
        } else {
            let time_start = SystemTime::now();

            problem_b(n, a, b);

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
