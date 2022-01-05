use std::{
    io::{
        stdin,
        Result as IOResult
    },
    str::{
        FromStr
    }
};

#[derive(Debug, Eq, PartialEq)]
pub enum ProblemAResult {
    Odd,
    Even
}

impl From<i32> for ProblemAResult {
    fn from(n: i32) -> Self {
        if (n & 1) == 0 {
            ProblemAResult::Even
        } else {
            ProblemAResult::Odd
        }
    }
}

pub fn main() -> IOResult<()> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let input_trimmed = input.trim_end();
    let mut input_split = input_trimmed.split(' ');
    let a = if let Some(ref s) = input_split.next() {
        i32::from_str(s).expect("'a' isn't a number!")
    } else {
        panic!("'a' isn't input!");
    };
    let b = if let Some(ref s) = input_split.next() {
        i32::from_str(s).expect("'b' isn't a number!")
    } else {
        panic!("'b' isn't input!");
    };

    println!("{:?}", problem_a(a, b));
    Ok(())
}

pub fn problem_a(a: i32, b: i32) -> ProblemAResult {
    (a * b).into()
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
    fn test_problem_a(a: i32, b: i32) -> TestResult {
        let result_a: ProblemAResult = a.into();
        let result_b: ProblemAResult = b.into();

        if !((1 <= a && a <= 10000) && (1 <= b && b <= 10000)) {
            TestResult::discard()
        } else if !((result_b == ProblemAResult::Even) && (problem_a(a, b) == ProblemAResult::Even)) {
            TestResult::failed()
        } else if !((result_a == ProblemAResult::Odd) && (result_b == ProblemAResult::Odd) && (problem_a(a, b) == ProblemAResult::Odd)) {
            TestResult::failed()
        } else {
            TestResult::passed()
        }
    }

    #[test]
    fn test_execution_time() -> Result<(), SystemTimeError> {
        let time_start = SystemTime::now();

        problem_a(1, 1);

        let time_end = SystemTime::now();
        let time_diff = time_end.duration_since(time_start)?;

        assert!(time_diff.as_secs() <= 2, "The code of ABC086A exceeded the time limit!");
        Ok(())
    }
}
