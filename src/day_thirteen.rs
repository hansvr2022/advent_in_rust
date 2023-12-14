#[cfg(test)]
mod day_thirteen {
    use std::ops::Add;

    #[test]
    fn test() {
        let mut lines = include_str!("../resources/input_day_thirteen.txt").lines();
        let mut fields: Vec<Vec<Vec<char>>> = Vec::new();

        while lines.clone().count() > 0 {
            let lines_taken_horizontal: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).map(|line| {
                let chars = line.chars();
                return chars.collect::<Vec<_>>();
            }).collect();
            let mut field: Vec<Vec<char>> = Vec::new();
            for i in 0..lines_taken_horizontal.len() {
                field.insert(i, lines_taken_horizontal[i].iter().map(|x| *x).collect());
            }
            fields.push(field);
        }

        let result = go_through_fields(fields.clone());
        println!("end result: {:?}", result.0 * 100 + result.1);
    }

    fn go_through_fields(fields: Vec<Vec<Vec<char>>>) -> (u64, u64) {
        let mut row_total: u64 = 0;
        let mut column_total: u64 = 0;


        'field_loop: for field in fields {
            let field_width = field[0].len();
            let field_height = field.len();
            let mut values: Vec<String> = Vec::new();
            for row in &field {
                values.push(row.iter().map(|x| x.to_string()).reduce(|x, y| x.add(&*y)).expect("wrong"));
            }
            'row_loop: for i in 0..values.len() - 1 {
                let mut smudges_count = number_of_different_chars(&mut values, i, i + 1);
                if smudges_count <= 1 {
                    let mut k = i + 2;
                    if i >= 1 {
                        'mirror_check: for j in (0..i).rev() {
                            if k >= values.len() {
                                break 'mirror_check;
                            }
                            if values[j] != values[k] {
                                if smudges_count == 0 && number_of_different_chars(&mut values, j, k) == 1 {
                                    smudges_count += 1;
                                } else {
                                    continue 'row_loop;
                                }
                            }
                            k += 1;
                        }
                    }
                    if smudges_count == 1 {
                        row_total += u64::try_from(i + 1).expect("should not go wrong");
                        continue 'field_loop;
                    }
                }
            }
            values = Vec::new();
            for column_index in 0..field_width {
                let mut number_to_push = String::new();
                for i in 0..field_height {
                    number_to_push = number_to_push.add(field[i][column_index].to_string().as_str());
                }
                values.push(number_to_push.parse().unwrap());
            }
            'column: for i in 0..values.len() - 1 {
                let mut smudges_count = number_of_different_chars(&mut values, i, i + 1);
                if smudges_count <= 1 {
                    // found the middle
                    // extra check
                    let mut k = i + 2;
                    if i >= 1 {
                        'mirror_check: for j in (0..i).rev() {
                            if k == values.len() {
                                break 'mirror_check;
                            }
                            if values[j] != values[k] {
                                if smudges_count == 0 && number_of_different_chars(&mut values, j, k) == 1 {
                                    smudges_count += 1;
                                } else {
                                    continue 'column;
                                }
                            }
                            k += 1;
                        }
                    }
                    if smudges_count == 1 {
                        column_total += u64::try_from(i + 1).expect("should not go wrong");
                        continue 'field_loop;
                    }
                }
            }
            println!("should not end up here with this field {:?}", field)
        }
        return (row_total, column_total);
    }

    fn number_of_different_chars(mut values: &mut Vec<String>, mut k: usize, j: usize) -> usize {
        return values[k].chars().zip(values[j].chars()).filter(|(q, z)| q != z).count();
    }
}