#[cfg(test)]
mod day_nine {
    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_nine.txt").lines();
        let mut total = 0;

        for line in lines {
            let numbers: Vec<_>= line.split(' ').rev().map(|x| i128::from_str_radix(x,10).expect("no error")).collect();
            let mut sequences: Vec<Vec<i128>> = Vec::new();
            sequences.push(numbers.clone());
            let mut current = numbers.clone();
            while !current.iter().all(|x| x.eq(&0)) {
                let mut diffs: Vec<_> = Vec::new();
                for i in 0..(current.len() -1) {
                    diffs.push(current[i + 1] - current[i])
                }
                current = diffs.clone();
                sequences.push(diffs);
            }
            for i in (1..sequences.len()).rev() {
                let next = sequences[i].last().expect("") + sequences[i - 1].last().expect("");
                sequences[i - 1].push(next);
            }
            total += sequences[0].last().expect("should be there");
        }

        println!("total: {}", total)

    }
}