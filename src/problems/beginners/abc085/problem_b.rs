use std::{
    io::{
        Error as IOError,
        ErrorKind,
        Result as IOResult,
        stdin
    },
    str::{
        FromStr
    }
};

pub fn main() -> IOResult<()> {
    let mut line_number = String::new();

    stdin().read_line(&mut line_number)?;

    let line_number_trimmed = line_number.trim_end();
    let n = usize::from_str(line_number_trimmed).expect("'N' isn't a number!");
    let mut mochis: Vec<u8> = Vec::new();

    for i in 0..n {
        let mut line_mochi = String::new();

        stdin().read_line(&mut line_mochi)?;

        let line_mochi_trimmed = line_mochi.trim_end();
        let mut mochi = u8::from_str(line_mochi_trimmed).expect(format!("{} isn't a number!", line_mochi_trimmed).as_str());

        mochis.push(mochi);
    }

    if n != mochis.len() {
        Err(IOError::new(ErrorKind::InvalidInput, "N isn't equal to actual mochis length!"))
    } else {
        println!("{}", problem_b(mochis));
        Ok(())
    }
}

pub fn problem_b(mut mochis: Vec<u8>) -> usize {
    mochis.sort_by(|a, b| a.cmp(b));
    mochis.dedup();
    mochis.len()
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_b(mochis: Vec<u8>) -> TestResult {
        if mochis.len() < 1 || mochis.len() > 100 {
            TestResult::discard()
        } else {
            for i in 0..mochis.len() {
                if mochis[i] < 1 || mochis[i] > 100 {
                    return TestResult::discard();
                }
            }

            let mut _mochis = mochis.clone();

            _mochis.sort_by(|a, b| a.cmp(b));
            _mochis.dedup();

            if _mochis.len() == problem_b(mochis) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(mochis: Vec<u8>) -> TestResult {
        if mochis.len() < 1 || mochis.len() > 100 {
            TestResult::discard()
        } else {
            for i in 0..mochis.len() {
                if mochis[i] < 1 || mochis[i] > 100 {
                    return TestResult::discard();
                }
            }

            let time_start = SystemTime::now();

            problem_b(mochis);

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
