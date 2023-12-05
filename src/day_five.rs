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
        let mut all_the_mappers: HashMap<String, Vec<Mapper>> = HashMap::new();
        all_the_mappers.insert(String::from("seed-to-soil"), Vec::new());
        all_the_mappers.insert(String::from("soil-to-fertilizer"), Vec::new());
        all_the_mappers.insert(String::from("fertilizer-to-water"), Vec::new());
        all_the_mappers.insert(String::from("water-to-light"), Vec::new());
        all_the_mappers.insert(String::from("light-to-temperature"), Vec::new());
        all_the_mappers.insert(String::from("temperature-to-humidity"), Vec::new());
        all_the_mappers.insert(String::from("humidity-to-location"), Vec::new());
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
                let mut seeds_after_range: Vec<Vec<u64>>= Vec::new();
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
                let mapper = Mapper {
                    is_for: String::from(destination_map),
                    destination_range_start: numbers[0],
                    source_range_start: numbers[1],
                    range_end: numbers[1] + numbers[2] - 1,
                    range_length: numbers[2],
                };
                if all_the_mappers.contains_key(destination_map) {
                    all_the_mappers.get_mut(destination_map).unwrap().push(mapper)
                } else {
                    let mappers = vec![mapper];
                    all_the_mappers.insert(destination_map.parse().unwrap(), mappers);
                }
            }
        }
        let mut min_max_per_category: HashMap<String, (u64, u64)> = HashMap::new();
        all_the_mappers.iter().for_each(|x| {
            let min_max =  x.1.iter().map(|vecy| (vecy.source_range_start, vecy.range_end)).reduce(|a,b| {
                let q = if a.0 < b.0 {  a.0 } else { b.0 };
                let x = if a.1 > b.1 {  a.1 } else { b.1 };
                let tuple = (q, x);
                return tuple;
            }).unwrap();
            min_max_per_category.insert(String::from(x.0), min_max);
        });

        println!("start with this amount of seeds: {}", seeds.len());
        let lowest = seeds.par_iter().map(|seeds| {
            seeds.par_iter().map(|seed| {
            let mut current_source = *seed;
            'mappers_loop: for key in &keys {
                let min_max = min_max_per_category.get(*key).unwrap();
                if min_max.0 <= current_source && min_max.1 >= current_source {
                    for mapper in all_the_mappers.get(*key).unwrap() {
                        if mapper.source_range_start <= current_source && current_source <= mapper.range_end {
                            current_source = mapper.destination_range_start + current_source - mapper.source_range_start;
                            continue 'mappers_loop;
                        }
                    }
                }
            }
            return current_source;
        }).min()}).min();

        println!("lowest: {}", lowest.unwrap().unwrap());
    }

    struct Mapper {
        is_for: String,
        destination_range_start: u64,
        source_range_start: u64,
        range_end: u64,
        range_length: u64,
    }
}