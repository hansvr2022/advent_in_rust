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
            let mut first_digit: (usize, u32) = (usize::MIN, 0);
            let mut last_digit: (usize, u32) = (usize::MIN, 0);

            for string_digit in &string_digits {
                let string_to_match = *string_digit.0;
                let matched: Vec<_> = line.match_indices(string_to_match).collect();
                if !matched.is_empty() {
                    for filtered_matched in [matched.first().expect("not found"), matched.last().expect("not found")] {
                        if first_digit.1 == 0 || filtered_matched.0 <= first_digit.0 {
                            first_digit = (filtered_matched.0, *string_digit.1)
                        }
                        if last_digit.1 == 0 || filtered_matched.0 >= last_digit.0 {
                            last_digit = (filtered_matched.0, *string_digit.1)
                        }
                    }
                }
            }

            total += (first_digit.1 * 10) + last_digit.1;
        }

        println!("total: {}", total)
    }
}
