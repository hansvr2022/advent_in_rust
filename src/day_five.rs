#[cfg(test)]
mod day_five {
    use std::cmp::{max, min};
    use std::collections::HashMap;
    use std::ops::Range;
    use std::str::FromStr;

    use rayon::iter::*;

    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_five.txt").lines();
        let mut seeds: Vec<Range<i64>> = Vec::new();
        let mut destination_map = "seed to soil";
        let mut all_the_mappers: HashMap<String, Vec<Mapper>> = HashMap::new();
        all_the_mappers.insert(String::from("seed-to-soil"), Vec::new());
        all_the_mappers.insert(String::from("soil-to-fertilizer"), Vec::new());
        all_the_mappers.insert(String::from("fertilizer-to-water"), Vec::new());
        all_the_mappers.insert(String::from("water-to-light"), Vec::new());
        all_the_mappers.insert(String::from("light-to-temperature"), Vec::new());
        all_the_mappers.insert(String::from("temperature-to-humidity"), Vec::new());
        all_the_mappers.insert(String::from("humidity-to-location"), Vec::new());


        for line in lines {
            if line.starts_with("seeds:") {
                let seeds_and_range: Vec<i64> = line.split(":").last().unwrap()
                    .trim().split(" ").map(|x| i64::from_str(x).unwrap()).collect();
                for i in 0..seeds_and_range.len() {
                    if i % 2 == 0 {
                        let to: i64 = seeds_and_range[i] + seeds_and_range[i + 1];
                        let seeds_to_add = seeds_and_range[i]..to;
                        seeds.push(seeds_to_add);
                    }
                }
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
                let numbers: Vec<_> = line.trim().split(" ").filter(|x| !x.is_empty()).map(|x| i64::from_str(x).unwrap()).collect();
                all_the_mappers.get_mut(destination_map).unwrap().push(Mapper { delta: numbers[1] - numbers[0], range: numbers[1]..(numbers[1] + numbers[2]) });
            }
        }

        println!("seeds: {:?}", seeds);
        let ranges: Vec<_> = all_the_mappers.values().map(|x| x.first().unwrap()).map(|x| x.range.start).collect();
        let deltas: Vec<_> = all_the_mappers.values().map(|x| x.first().unwrap()).map(|x| x.delta).zip(ranges).collect();
        println!("mappers: {:?}", deltas);

        let results: Vec<_> = seeds.par_iter()
            .map(|x| whoop(x, &all_the_mappers, "seed-to-soil")).flatten()
            .map(|x| whoop(&x, &all_the_mappers, "soil-to-fertilizer")).flatten()
            .map(|x| whoop(&x, &all_the_mappers, "fertilizer-to-water")).flatten()
            .map(|x| whoop(&x, &all_the_mappers, "water-to-light")).flatten()
            .map(|x| whoop(&x, &all_the_mappers, "light-to-temperature")).flatten()
            .map(|x| whoop(&x, &all_the_mappers, "temperature-to-humidity")).flatten()
            .map(|x| whoop(&x, &all_the_mappers, "humidity-to-location")).flatten().collect();

        let smallest = results.iter().map(|t: &Range<i64>| t.start).min();


        println!("lowest: {:?}", smallest);
    }

    fn whoop(range: &Range<i64>, all_the_mappers: &HashMap<String, Vec<Mapper>>, key: &str) -> Vec<Range<i64>> {
        let start = range.start;
        let end = range.end;
        let mut result: Vec<_> = all_the_mappers.get(key).unwrap().iter()
            .filter(move |m| start <= m.range.end && end >= m.range.start)
            .map(move |m| (max(start, m.range.start) - m.delta)..(min(end, m.range.end) - m.delta)).collect();
        let min_result = result.iter().map(|x| x.start).min();
        let max_result = result.iter().map(|x| x.end).max();
        if result.is_empty() {
            return vec![start..end];
        }

        return result;
    }

    struct Mapper {
        delta: i64,
        range: Range<i64>,
    }

    impl Mapper {}
}