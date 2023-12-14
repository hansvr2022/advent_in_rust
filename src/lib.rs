mod day_two;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_thirteen;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn day_one() {
        let file = fs::read_to_string("./resources/input_day_one.txt").expect("should have worked");
        let mut total = 0;
        let string_digits: HashMap<&str, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);

        for line in file.lines() {
            let mut first_digit: (usize, u32) = (0, 0);
            let mut last_digit: (usize, u32) = (0, 0);

            for string_digit in &string_digits {
                let string_to_match = *string_digit.0;
                let matched: Vec<_> = line.match_indices(string_to_match).collect();
                if !matched.is_empty() {
                    let first = matched.first().expect("no error");
                    let last = matched.last().expect("no error");
                    if first_digit.0 == 0 || first.0 <= first_digit.0 {
                        first_digit = (first.0, *string_digit.1)
                    }
                    if last.0 >= last_digit.0 {
                        last_digit = (last.0, *string_digit.1)
                    }
                }
            }

            total += (first_digit.1 * 10) + last_digit.1;
        }

        println!("total: {}", total)
    }
}
