#[cfg(test)]
mod day_six {
    use std::str::{FromStr, Lines};

    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_six.txt").lines();

        let result = part_one(lines);
        let result_part_two = part_two();


        println!("result part one: {}", result);
        println!("result part two: {}", result_part_two);
    }

    fn part_two() -> usize {
        return 38_981_510 - 10_007_573 - 1;
    }

    fn part_one(lines: Lines) -> usize {
        let mut times: Vec<u64> = Vec::new();
        let mut record_distances: Vec<u64> = Vec::new();
        for line in lines {
            if line.starts_with("Time") {
                times = line.split(":").last().unwrap()
                    .trim().split(" ").filter(|x| !x.is_empty())
                    .map(|x| u64::from_str(x).expect("should have been a number"))
                    .collect();
            }
            if line.starts_with("Distance") {
                record_distances = line.split(":").last().unwrap()
                    .trim().split(" ").filter(|x| !x.is_empty())
                    .map(|x| u64::from_str(x).expect("should have been a number"))
                    .collect();
            }
        }
        let mut races: Vec<Race> = Vec::new();
        for i in 0..times.len() {
            let race = Race { max_time: times[i], record_distance: record_distances[i], held_for_distance: 0 };
            races.push(race);
        }

        let result: usize = races.iter().map(|race| {
            let mut winners: Vec<u64> = Vec::new();
            for i in 1..race.max_time {
                let distance = race.calculate_distance(i);
                if distance > race.record_distance {
                    println!("winner distance: {}, held for {}, old record: {}", distance, i, race.record_distance);
                    winners.push(i);
                }
            }
            return winners.len();
        }).reduce(|x: usize, y: usize| x * y).expect("should be fine");
        result
    }

    struct Race {
        max_time: u64,
        record_distance: u64,
        held_for_distance: u64,
    }

    impl Race {
        fn calculate_distance(&self, seconds_held: u64) -> u64 {
            if seconds_held > self.max_time { return 0; }
            let distance = (self.max_time - seconds_held) * seconds_held;
            return distance;
        }

        fn calculate_held_for_distance(&self, distance: u32) -> u32 {
           return 0;
        }
    }
}