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
    let mut line_number_cards = String::new();
    let mut line_cards = String::new();

    stdin().read_line(&mut line_number_cards)?;
    stdin().read_line(&mut line_cards)?;

    let line_number_cards_trimmed = line_number_cards.trim_end();
    let line_cards_trimmed = line_cards.trim_end();
    let n = usize::from_str(line_number_cards_trimmed).expect("'N' isn't a number!");
    let line_cards_split = line_cards_trimmed.split(" ");
    let mut cards: Vec<i16> = Vec::new();

    for s in line_cards_split {
        let card = i16::from_str(s).expect(format!("{} isn't a number!", s).as_str());

        cards.push(card);
    }

    if n != cards.len() {
        Err(IOError::new(ErrorKind::InvalidInput, "N ins't equal to actual cards length!"))
    } else {
        println!("{}", problem_b(cards));
        Ok(())
    }
}

pub fn problem_b(mut cards: Vec<i16>) -> i16 {
    let mut alice_total: i16 = 0;
    let mut bob_total: i16 = 0;

    cards.sort_by(|a, b| a.cmp(b));

    while !cards.is_empty() {
        alice_total += cards.pop().unwrap_or(0);
        bob_total += cards.pop().unwrap_or(0);
    }

    alice_total - bob_total
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use quickcheck::TestResult;
    use super::*;

    #[quickcheck]
    fn test_problem_b(mut cards: Vec<i16>) -> TestResult {
        if cards.len() < 1 || cards.len() > 100 {
            TestResult::discard()
        } else {
            for i in 0..cards.len() {
                if cards[i] < 1 || cards[i] > 100 {
                    return TestResult::discard();
                }
            }

            cards.sort_by(|a, b| a.cmp(b));

            let mut alice_total: i16 = 0;
            let mut bob_total: i16 = 0;

            while !cards.is_empty() {
                alice_total += cards.pop().unwrap_or(0);
                bob_total += cards.pop().unwrap_or(0);
            }

            if (alice_total - bob_total) == problem_b(cards) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }
    }

    #[quickcheck]
    fn test_execution_time(cards: Vec<i16>) -> TestResult {
        if cards.len() < 1 || cards.len() > 100 {
            TestResult::discard()
        } else {
            for i in 0..cards.len() {
                if cards[i] < 1 || cards[i] > 100 {
                    return TestResult::discard();
                }
            }

            let time_start = SystemTime::now();

            problem_b(cards);

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
