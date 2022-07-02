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
    let mut line_number_ints = String::new();
    let mut line_ints = String::new();

    stdin().read_line(&mut line_number_ints)?;

    let line_number_ints_trimmed = line_number_ints.trim_end();
    let number_ints = i32::from_str(line_number_ints_trimmed).expect("'n' isn't a number");

    stdin().read_line(&mut line_ints)?;

    let mut ints = Vec::new();

    for s in line_ints.trim_end().split(' ') {
        let int = i32::from_str(s).expect(format!("{} isn't a number!", s).as_str());

        ints.push(int);
    }

    if (number_ints as usize) != ints.len() {
        Err(IOError::new(ErrorKind::InvalidInput, "N isn't equal to actual integers length!"))
    } else {
        println!("{}", problem_b(ints));
        Ok(())
    }
}

pub fn problem_b(mut ints: Vec<i32>) -> i32 {
    let mut order: i32 = 0;
    let mut has_odd = false;

    loop {
        for i in 0..ints.len() {
            if ints[i] & 1 != 0 {
                has_odd = true;
                break;
            } else {
                ints[i] >>= 1;
            }
        }

        if has_odd {
            break;
        } else {
            order += 1;
        }
    }

    order
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_b(ints: Vec<i32>) -> TestResult {
        if ints.len() < 1 || ints.len() > 200 {
            TestResult::discard()
        } else {
            for i in 0..ints.len() {
                if ints[i] < 1 || ints[i] > 1000000000 {
                    return TestResult::discard();
                }
            }

            let mut current = ints.clone();
            let mut order: i32 = 0;

            loop {
                current = current.into_iter().take_while(|i| i & 1 == 0).collect();

                if current.len() == ints.len() {
                    order += 1;
                    current = current.into_iter().map(|i| i >> 1).collect();
                } else {
                    break;
                }
            }

            if order == problem_b(ints) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(ints: Vec<i32>) -> TestResult {
        if ints.len() < 1 || ints.len() > 200 {
            TestResult::discard()
        } else {
            for i in 0..ints.len() {
                if ints[i] < 1 || ints[i] > 1000000000 {
                    return TestResult::discard();
                }
            }

            let time_start = SystemTime::now();

            problem_b(ints);

            let time_end = SystemTime::now();
            let time_diff = time_end.duration_since(time_start).expect("Something wrong!");

            if time_diff.as_secs() <= 2 {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }
}
