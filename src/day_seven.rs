#[cfg(test)]
mod day_six {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::str::{FromStr, Lines};

    use crate::day_seven::day_six::TypeOfPlay::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPair};

    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_seven.txt").lines();
        let mut plays = extract_cards(lines);
        plays.sort_by(|a, b| a.cmp(b));

        // for play in plays {
        //     println!("play:  {}, bid: {}" , play.type_of_play.as_str(), play.bid);
        // }
        let mut total = 0;
        for i in 0..plays.len() {
                println!("play:  {}, bid: {}" , plays[i].type_of_play.as_str(), plays[i].bid);
            total += (i as u32 + 1) * plays[i].bid;
        }

        print!("total {}", total);
    }



    fn extract_cards(input: Lines) -> Vec<Play> {
        let mut plays: Vec<Play> = Vec::new();

        let mut cards_map: HashMap<char, PlayCard> = HashMap::new();
        cards_map.insert('2', PlayCard{ value: 2, name: String::from("2")});
        cards_map.insert('3', PlayCard{ value: 3, name: String::from("3")});
        cards_map.insert('4', PlayCard{ value: 4, name: String::from("4")});
        cards_map.insert('5', PlayCard{ value: 5, name: String::from("5")});
        cards_map.insert('6', PlayCard{ value: 6, name: String::from("6")});
        cards_map.insert('7', PlayCard{ value: 7, name: String::from("7")});
        cards_map.insert('8', PlayCard{ value: 8, name: String::from("8")});
        cards_map.insert('9', PlayCard{ value: 9, name: String::from("9")});
        cards_map.insert('T', PlayCard{ value: 10, name: String::from("Ten")});
        cards_map.insert('J', PlayCard{ value: 1, name: String::from("Joker")});
        cards_map.insert('Q', PlayCard{ value: 12, name: String::from("Queen")});
        cards_map.insert('K', PlayCard{ value: 13, name: String::from("King")});
        cards_map.insert('A', PlayCard{ value: 14, name: String::from("Aces")});

        for line in input {
            let line_splitted: Vec<_> = line.split(" ").collect();
            let mut cards_in_play: Vec<PlayCard> = Vec::new();
            for card in line_splitted[0].chars() {
                cards_in_play.push(cards_map.get(&card).expect("shouuld have excisted").clone());
            }
            plays.push(Play{ cards: cards_in_play.clone(), bid: u32::from_str(line_splitted[1]).unwrap(), type_of_play: Play::determine_type_of_play(cards_in_play)})
        }
        return plays;
    }

    #[derive(Eq, PartialEq, PartialOrd)]
    struct Play {
        cards: Vec<PlayCard>,
        type_of_play: TypeOfPlay,
        bid: u32,
    }

    impl Ord for Play {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.type_of_play > other.type_of_play {
                return Ordering::Greater;
            }
            if self.type_of_play < other.type_of_play {
                return Ordering::Less
            }
            for i in 0..self.cards.len() {
                if self.cards[i].value > other.cards[i].value {
                    return Ordering::Greater
                } else if self.cards[i].value < other.cards[i].value {
                    return Ordering::Less
                }
            }
            return Ordering::Equal
        }
    }
    impl Play {
        fn determine_type_of_play(mut cards: Vec<PlayCard>) -> TypeOfPlay {
            let mut sorted =  cards.as_mut_slice();
            sorted.sort_by_key(|x| x.value);
            let mut values: HashMap<u32, u32> = HashMap::new();
            for card in sorted {
                *values.entry(card.value).or_insert(0) += 1;
            }
            if values.len() == 1 {
                return FiveOfAKind
            }
            if values.len() == 5 {
                if values.contains_key(&1) {
                    println!("pair returned");
                    return Pair }
                return HighCard
            }
            let mut values_vec: Vec<(u32, u32)> = values.iter().map(|x| (*x.0, *x.1)).collect();
            values_vec.sort_by(|a, b| a.1.cmp(&b.1));
            values_vec.reverse();
            if values_vec[0].1 == 4 {
                if values_vec[1].0 == 1 || values_vec[0].0 == 1 { return FiveOfAKind }
                return FourOfAKind
            }
            if values_vec[0].1.eq(&3) && values_vec[1].1 == 2 {
                if values_vec.iter().any(|x| x.0 == 1) { return FiveOfAKind }
                return FullHouse
            }
            if values_vec[0].1 == 3 {
                if values_vec[0].0 == 1 || values_vec[1].0 == 1 || values_vec[2].0 == 1 { return FourOfAKind }
                return ThreeOfAKind
            }
            if values_vec[0].1 == 2 && values_vec[1].1 == 2 {
                if values_vec[1].0 == 1 || values_vec[0].0 == 1 {
                    return FourOfAKind
                }
                if values_vec[2].0 == 1 {
                    println!("Fullhouse returned");
                    return FullHouse;
                }
                return TwoPair
            }
            if values_vec[0].1 == 2 {
                if values_vec[0].0 == 1 {
                    return ThreeOfAKind;
                }
                for i in 1..values_vec.len() {
                    if values_vec[i].0 == 1 {
                        return ThreeOfAKind
                    }
                }
                return Pair
            }
            return HighCard
        }
    }
    #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
    struct PlayCard {
        value: u32,
        name: String,
    }

    #[derive(Eq, PartialEq, Ord, PartialOrd)]
    enum TypeOfPlay {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl TypeOfPlay {
        fn as_str(&self) -> &'static str {
            match self {
                FiveOfAKind => "Five of a kind",
                FourOfAKind => "Four of a kind",
                TwoPair => "Two pair",
                FullHouse => "Full House",
                Pair => "Pair",
                HighCard => "High card",
                ThreeOfAKind => "Three of a kind",
            }
        }
    }
}