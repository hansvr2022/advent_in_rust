#[cfg(test)]
mod day_five {
    use std::collections::HashMap;
    use std::str::FromStr;
    use rayon::iter::*;

    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_five.txt").lines();
        let mut seeds: Vec<Vec<u64>> = Vec::new();
        let mut destination_map = "seed to soil";
        let mut all_the_mappers: HashMap<String, HashMap<u64, u64>> = HashMap::new();
        all_the_mappers.insert(String::from("seed-to-soil"), HashMap::new());
        all_the_mappers.insert(String::from("soil-to-fertilizer"), HashMap::new());
        all_the_mappers.insert(String::from("fertilizer-to-water"), HashMap::new());
        all_the_mappers.insert(String::from("water-to-light"), HashMap::new());
        all_the_mappers.insert(String::from("light-to-temperature"), HashMap::new());
        all_the_mappers.insert(String::from("temperature-to-humidity"), HashMap::new());
        all_the_mappers.insert(String::from("humidity-to-location"), HashMap::new());
        let keys = vec!["seed-to-soil",
                        "soil-to-fertilizer",
                        "fertilizer-to-water",
                        "water-to-light",
                        "light-to-temperature",
                        "temperature-to-humidity",
                        "humidity-to-location"];


        for line in lines {
            if line.starts_with("seeds:") {
                let seeds_and_range: Vec<u64> = line.split(":").last().unwrap()
                    .trim().split(" ").map(|x| u64::from_str(x).unwrap()).collect();
                let mut seeds_after_range: Vec<Vec<u64>> = Vec::new();
                for i in 0..seeds_and_range.len() {
                    if i % 2 == 0 {
                        let to = seeds_and_range[i] + seeds_and_range[i + 1] - 1;
                        let seeds_to_add: Vec<u64> = (seeds_and_range[i]..to).collect();
                        seeds_after_range.push(seeds_to_add);
                    }
                }
                seeds = seeds_after_range;
            } else if line.starts_with("seed-to-soil") {
                destination_map = "seed-to-soil";
            } else if line.starts_with("soil-to-fertilizer") {
                destination_map = "soil-to-fertilizer";
            } else if line.starts_with("fertilizer-to-water") {
                destination_map = "fertilizer-to-water";
            } else if line.starts_with("water-to-light") {
                destination_map = "water-to-light";
            } else if line.starts_with("light-to-temperature") {
                destination_map = "light-to-temperature";
            } else if line.starts_with("temperature-to-humidity") {
                destination_map = "temperature-to-humidity";
            } else if line.starts_with("humidity-to-location") {
                destination_map = "humidity-to-location";
            } else if line.is_empty() {} else {
                let numbers: Vec<_> = line.trim().split(" ").filter(|x| !x.is_empty()).map(|x| u64::from_str(x).unwrap()).collect();
                let mut hash: HashMap<u64, u64> = HashMap::new();
                for i in 0..(numbers[2] - 1) {
                    hash.insert(numbers[1] + i, numbers[0] + i);
                }
                if all_the_mappers.contains_key(destination_map) {
                    all_the_mappers.get_mut(destination_map).unwrap().extend(hash);
                } else {
                    all_the_mappers.insert(destination_map.parse().unwrap(), hash);
                }
            }
        }


        println!("start with this amount of seeds: {}", seeds.len());
        let lowest = seeds.par_iter().map(|seeds| {
            seeds.par_iter().map(|seed| {
                let mut current_source = *seed;
                for key in &keys {
                    current_source = *all_the_mappers.get(*key).unwrap().get(&current_source).unwrap_or(&current_source);
                }
                return current_source;
            }).min()
        }).min();

        println!("lowest: {}", lowest.unwrap().unwrap());
    }

    struct Mapper {
        is_for: String,
        swap: HashMap<u64, u64>,
    }
}