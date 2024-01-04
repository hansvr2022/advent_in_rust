#[cfg(test)]
mod day_thirteen {
    use std::collections::VecDeque;

    #[test]
    fn test() {
        let mut lines = include_str!("../resources/input_day_fourteen.txt").lines();
        let mut field: Vec<Vec<char>> = Vec::new();
        for line in lines {
            field.push(line.chars().collect());
        }

        let heaviest = field.len();
        let mut total_weight = 0;
        let mut same_weights = 0;
        let mut cycle_pattern: Vec<usize> = Vec::new();


        'looptieloop: for i in 0..1_000_000_000 {
            field = cycle(field);
            let old_total = total_weight;
            total_weight = calculate_weight_of_field(&field);
            cycle_pattern.push(total_weight);
        }

        println!("pattern: {:?}", cycle_pattern);
        let start_of_cycle: usize = 99356;
        let first = cycle_pattern.iter().position(|x| x == &start_of_cycle).unwrap();
        let second = cycle_pattern.iter().skip(first + 5).position(|x| x == &start_of_cycle).unwrap();
        println!("first: {:?}, {:?}", first, second);
        let length_of_cycle = second - first;
        let number_of_cycles = 10_000_000_000 % length_of_cycle;
        println!("cycles: {}", number_of_cycles);

        println!("{}", cycle_pattern[first + number_of_cycles]);

    }

    fn calculate_weight_of_field(field: &Vec<Vec<char>>) -> usize {
        let mut weight = 0;
        let mut max_weight = field.len();
        for column in 0..field[0].len() {
            for row in 0..field.len() {
                if field[row][column] == 'O' {
                    weight += max_weight - row;
                }
            }
        }
        return weight;
    }

    fn cycle(mut field: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut empty_spots_queue: VecDeque<usize> = VecDeque::new();
        //north
        for x in 0..field[0].len() {
            empty_spots_queue.clear();
            for y in 0..field.len() {
                if field[y][x] == '.' {
                    empty_spots_queue.push_back(y);
                }
                if field[y][x] == '#' {
                    empty_spots_queue.clear()
                }
                if field[y][x] == 'O' {
                    let frond = empty_spots_queue.pop_front();
                    if frond.is_some() {
                        field[frond.unwrap()][x] = 'O';
                        field[y][x] = '.';
                        empty_spots_queue.push_back(y);
                    }
                }
            }
        }
        // west
        empty_spots_queue.clear();
        for y in 0..field.len() {
            empty_spots_queue.clear();
            for x in 0..field[0].len() {
                if field[y][x] == '.' {
                    empty_spots_queue.push_back(x);
                }
                if field[y][x] == '#' {
                    empty_spots_queue.clear()
                }
                if field[y][x] == 'O' {
                    let frond = empty_spots_queue.pop_front();
                    if frond.is_some() {
                        field[y][frond.unwrap()] = 'O';
                        field[y][x] = '.';
                        empty_spots_queue.push_back(x);
                    }
                }
            }
        }
        // south
        empty_spots_queue.clear();
        for x in 0..field[0].len() {
            empty_spots_queue.clear();
            for y in (0..field.len()).rev() {
                if field[y][x] == '.' {
                    empty_spots_queue.push_back(y);
                }
                if field[y][x] == '#' {
                    empty_spots_queue.clear()
                }
                if field[y][x] == 'O' {
                    let frond = empty_spots_queue.pop_front();
                    if frond.is_some() {
                        field[frond.unwrap()][x] = 'O';
                        field[y][x] = '.';
                        empty_spots_queue.push_back(y);
                    }
                }
            }
        }

        // east
        empty_spots_queue.clear();
        for y in 0..field.len() {
            empty_spots_queue.clear();
            for x in (0..field[0].len()).rev() {
                if field[y][x] == '.' {
                    empty_spots_queue.push_back(x);
                }
                if field[y][x] == '#' {
                    empty_spots_queue.clear()
                }
                if field[y][x] == 'O' {
                    let frond = empty_spots_queue.pop_front();
                    if frond.is_some() {
                        field[y][frond.unwrap()] = 'O';
                        field[y][x] = '.';
                        empty_spots_queue.push_back(x);
                    }
                }
            }
        }


        return field;
    }
}