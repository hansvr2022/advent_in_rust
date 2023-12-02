#[cfg(test)]
mod tests_day_two {
    use std::fs;
    use std::str::FromStr;
    use regex::Regex;

    #[test]
    fn day_two() {
        let file = fs::read_to_string("./resources/input_day_two.txt").expect("should have worked");
        let mut total = 0;
        for line in file.lines() {
            let game_regex = Regex::new(r"\d*:").unwrap();
            let game_match = game_regex.find(line).expect("nothing found").as_str().trim_end_matches(":");
            let greens: Vec<u32> = Regex::new(r"(\d* g)").unwrap().find_iter(line).map(|x| x.as_str().trim_end_matches(" g")).map(|x| u32::from_str(x).unwrap()).collect();
            let blues: Vec<u32> = Regex::new(r"(\d* b)").unwrap().find_iter(line).map(|x| x.as_str().trim_end_matches(" b")).map(|x| u32::from_str(x).unwrap()).collect();
            let reds: Vec<u32> = Regex::new(r"(\d* r)").unwrap().find_iter(line).map(|x| x.as_str().trim_end_matches(" r")).map(|x| u32::from_str(x).unwrap()).collect();
            let max_green = greens.iter().max().unwrap();
            let max_blue = blues.iter().max().unwrap();
            let max_red = reds.iter().max().unwrap();
            total += max_red * max_blue * max_green
        }

        println!("{}", total)
    }
}