#[cfg(test)]
mod day_eight {
    use std::collections::HashMap;
    use num::integer::lcm;
    use num::ToPrimitive;

    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_eight.txt").lines();
        let left_right_line: String = lines.clone().take(1).collect();
        let instructions: Vec<char> = left_right_line.chars().collect();
        let mut locations: HashMap<String, (String, String)> = HashMap::new();
        let mut current_positions: Vec<String> = Vec::new();
        let mut total_steps: u128 = 0;
        let mut pos_in_loop = 0;

        for line in lines.skip(2) {
            let key = line.get(0..3).unwrap().to_string();
            let first = line.get(7..10).unwrap().to_string();
            let second = line.get(12..15).unwrap().to_string();
            locations.insert(key.clone(), (first, second));
            if key.ends_with("A") {
                current_positions.push(key.clone());
            }
        }

        let mut final_steps: Vec<u128> = Vec::new();


        let finish: String = "ZZZ".to_string();

        for position in current_positions {
            let mut current = position.clone();
            while !current.ends_with("Z") {
                if pos_in_loop == instructions.len() {
                    pos_in_loop = 0;
                }

                let instruction = instructions[pos_in_loop];
                if instruction == 'L' {
                    current = locations[&current].0.to_string();
                } else {
                    current = locations[&current].1.to_string();
                }


                pos_in_loop += 1;
                total_steps += 1;
            }

            final_steps.push(total_steps);
            pos_in_loop = 0;
            total_steps = 0;
        }

        println!("steps: {:?}", final_steps);
        let result = final_steps.iter().map(|x| x.to_i128().expect("no error")).reduce(|a, b|  lcm(a, b)).expect("should have worked");
        println!("result: {}", result)
    }


}
