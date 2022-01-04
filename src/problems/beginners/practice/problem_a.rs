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
    let mut line_first = String::new();
    let mut line_second = String::new();
    let mut line_third = String::new();

    stdin().read_line(&mut line_first)?;
    stdin().read_line(&mut line_second)?;
    stdin().read_line(&mut line_third)?;

    let line_first_trimmed = line_first.trim_end();
    let line_second_trimmed = line_second.trim_end();
    let line_third_trimmed = line_third.trim_end();

    let a = i32::from_str(line_first_trimmed).expect("'a' isn't a number!");
    let mut line_second_split = line_second_trimmed.split(' ');
    let b = if let Some(ref s) = line_second_split.next() {
        i32::from_str(s).expect("'b' isn't a number!")
    } else {
        panic!("'b' isn't input!");
    };
    let c = if let Some(ref s) = line_second_split.next() {
        i32::from_str(s).expect("'c' isn't a number!")
    } else {
        panic!("'c' isn't input!");
    };
    let (sum, text) = problem_a(a, b, c, line_third_trimmed);

    println!("{} {}", sum, text);
    Ok(())
}

pub fn problem_a(a: i32, b: i32, c: i32, s: &str) -> (i32, &str) {
    (a + b + c, s)
}

#[cfg(test)]
mod tests {
    use std::{
        time::{
            SystemTime,
            SystemTimeError
        }
    };
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_a(a: i32, b: i32, c: i32, s: String) -> TestResult {
        let s_len = s.len();

        if !((1 <= a && a <= 1000) && (1 <= b && b <= 1000) && (1 <= c && c <= 1000) && (1 <= s_len && s_len <= 100)) {
            TestResult::discard()
        } else if problem_a(a, b, c, s.as_str()) != (a + b + c, s.as_str()) {
            TestResult::failed()
        } else {
            TestResult::passed()
        }
    }

    #[test]
    fn test_execution_time() -> Result<(), SystemTimeError> {
        let time_start = SystemTime::now();

        problem_a(1, 1, 1, &"me");

        let time_end = SystemTime::now();
        let time_diff = time_end.duration_since(time_start)?;

        assert!(time_diff.as_secs() <= 2, "The code of Practice A exceeded the time limit!");
        Ok(())
    }
}
