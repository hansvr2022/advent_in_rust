
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use super::*;


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
        ]);

        for line in file.lines() {
            let mut first_digit: (usize, u32) = (usize::MIN, 0);
            let mut last_digit: (usize, u32) = (usize::MIN, 0);

            for string_digit in &string_digits {
                let string_to_match = *string_digit.0;
                for matched in line.match_indices(string_to_match) {
                    if first_digit.1 == 0 || matched.0 <= first_digit.0 {
                        first_digit = (matched.0, *string_digit.1)
                    }
                    if last_digit.1 == 0 || matched.0 >= last_digit.0 {
                        last_digit = (matched.0, *string_digit.1)
                    }
                }
            }

            for (position, character) in line.char_indices() {
                if character.is_numeric() {
                    if first_digit.1 == 0 || position <= first_digit.0 {
                        first_digit = (position, character.to_digit(10).expect("not a digit"))
                    }
                    if last_digit.1 == 0 || position >= last_digit.0 {
                        last_digit = (position, character.to_digit(10).expect("not a digit"))
                    }
                }
            }

            total = total + (first_digit.1 * 10) + last_digit.1;
        }

        println!("total: {}", total)
    }
}
