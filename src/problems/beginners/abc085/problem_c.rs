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
    let str_n = input_split.next().expect("'N' isn't input.");
    let str_y = input_split.next().expect("'Y' isn't input.");
    let n = i32::from_str(str_n).expect("'N' isn't a number!");
    let y = i32::from_str(str_y).expect("'Y' isn't a number!");
    let (yen_10000, yen_5000, yen_1000) = problem_c(n, y);

    println!("{} {} {}", yen_10000, yen_5000, yen_1000);
    Ok(())
}

pub fn problem_c(n: i32, y: i32) -> (i32, i32, i32) {
    return loop {
        let mut _y = y;
        let yen_10000_exact = _y / 10000;

        _y %= 10000;

        let yen_5000_exact = _y / 5000;

        _y %= 5000;

        let yen_1000_exact = _y / 1000;

        if y == 10000 * yen_10000_exact + 5000 * yen_5000_exact + 1000 * yen_1000_exact && n == yen_10000_exact + yen_5000_exact + yen_1000_exact {
            break (yen_10000_exact, yen_5000_exact, yen_1000_exact);
        }

        let mut yen_10000: i32 = 0;
        let mut yen_5000: i32 = 0;
        let mut yen_1000: i32 = 0;
        let mut is_matched = false;
        let boundary_10000 = y / 10000 + 1;
        let boundary_5000 = y / 5000 + 1;
        let boundary_1000 = y / 1000 + 1;

        for i in 0..boundary_10000 {
            for j in 0..boundary_5000 {
                for k in 0..boundary_1000 {
                    if y == 10000 * i + 5000 * j + 1000 * k && n == i + j + k {
                        yen_10000 = i;
                        yen_5000 = j;
                        yen_1000 = k;
                        is_matched = true;
                        break;
                    }
                }

                if is_matched {
                    break;
                }
            }

            if is_matched {
                break;
            }
        }

        if is_matched {
            break (yen_10000, yen_5000, yen_1000);
        } else {
            break (-1, -1, -1);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_c(n: i32, y: i32) -> TestResult {
        if n < 1 || n > 2000 {
            TestResult::discard()
        } else if 1000 <= y && y <= 20000000 && y % 1000 != 0 {
            TestResult::discard()
        } else {
            let expected = loop {
                let mut _y = y;
                let yen_10000_exact = _y / 10000;

                _y %= 10000;

                let yen_5000_exact = _y / 5000;

                _y %= 5000;

                let yen_1000_exact = _y / 1000;

                if y == 10000 * yen_10000_exact + 5000 * yen_5000_exact + 1000 && n == yen_10000_exact + yen_5000_exact + yen_1000_exact {
                    break (yen_10000_exact, yen_5000_exact, yen_1000_exact);
                }

                let mut yen_10000: i32 = 0;
                let mut yen_5000: i32 = 0;
                let mut yen_1000: i32 = 0;
                let mut is_matched = false;
                let boundary_10000 = y / 10000 + 1;
                let boundary_5000 = y / 5000 + 1;
                let boundary_1000 = y / 1000 + 1;

                for i in 0..boundary_10000 {
                    for j in 0..boundary_5000 {
                        for k in 0..boundary_1000 {
                            if 10000 * i + 5000 * j + 1000 * k == y && n == i + j + k {
                                yen_10000 = i;
                                yen_5000 = j;
                                yen_1000 = k;
                                is_matched = true;
                                break;
                            }
                        }

                        if is_matched {
                            break;
                        }
                    }

                    if is_matched {
                        break;
                    }
                }

                if is_matched {
                    break (yen_10000, yen_5000, yen_1000);
                } else {
                    break (-1, -1, -1);
                }
            };

            if expected == problem_c(n, y) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(n: i32, y: i32) -> TestResult {
        if n < 1 || n > 2000 {
            TestResult::discard()
        } else if 1000 <= y && y <= 20000000 && y % 1000 != 0 {
            TestResult::discard()
        } else {
            let time_start = SystemTime::now();

            problem_c(n, y);

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
