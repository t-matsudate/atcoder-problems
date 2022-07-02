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
    let mut line_a = String::new();
    let mut line_b = String::new();
    let mut line_c = String::new();
    let mut line_x = String::new();

    stdin().read_line(&mut line_a)?;
    stdin().read_line(&mut line_b)?;
    stdin().read_line(&mut line_c)?;
    stdin().read_line(&mut line_x)?;

    let line_a_trimmed = line_a.trim_end();
    let line_b_trimmed = line_b.trim_end();
    let line_c_trimmed = line_c.trim_end();
    let line_x_trimmed = line_x.trim_end();
    let a = u16::from_str(line_a_trimmed).expect("'A' isn't a number!");
    let b = u16::from_str(line_b_trimmed).expect("'B' isn't a number!");
    let c = u16::from_str(line_c_trimmed).expect("'C' isn't a number!");
    let x = u16::from_str(line_x_trimmed).expect("'X' isn't a number!");

    println!("{}", problem_b(a, b, c, x));
    Ok(())
}

pub fn problem_b(a: u16, b: u16, c: u16, x: u16) -> u16 {
    let mut combinations: u16 = 0;

    for i in 0..(a + 1) {
        for j in 0..(b + 1) {
            for k in 0..(c + 1) {
                if 500 * i + 100 * j + 50 * k == x {
                    combinations += 1;
                }
            }
        }
    }

    combinations
}

#[cfg(test)]
mod tests {
    use std::time::{
        SystemTime
    };
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_b(a: u16, b: u16, c: u16, x: u16) -> TestResult {
        if a > 50 || b > 50 || c > 50 {
            TestResult::discard()
        } else if a + b + c < 1 {
            TestResult::discard()
        } else if x < 50 || x > 20000 {
            TestResult::discard()
        } else if x % 50 != 0 {
            TestResult::discard()
        } else {
            let combinations: u16 = (0..(a + 1)).fold(
                0,
                |ret, i| (0..(b + 1)).fold(
                    ret,
                    |ret, j| (0..(c + 1)).fold(
                        ret,
                        |ret, k| if 500 * i + 100 * j + 50 * k == x {
                            ret + 1
                        } else {
                            ret
                        }
                    )
                )
            );

            if combinations == problem_b(a, b, c, x) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(a: u16, b: u16, c: u16, x: u16) -> TestResult {
        if a > 50 || b > 50 || c >= 50 {
            TestResult::discard()
        } else if a + b + c < 1 {
            TestResult::discard()
        } else if x < 50 || x > 20000 {
            TestResult::discard()
        } else if x % 50 != 0 {
            TestResult::discard()
        } else {
            let time_start = SystemTime::now();

            problem_b(a, b, c, x);

            let time_end = SystemTime::now();
            let time_diff = time_end.duration_since(time_start).expect("Something wrong!");

            if time_diff.as_secs() < 2 {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }
}
