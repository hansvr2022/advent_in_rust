#[cfg(test)]
mod day_four {
    use std::str::FromStr;

    #[test]
    fn test() {
        let lines = include_str!("../resources/input_day_four.txt");
        let mut cards: Vec<Card> = Vec::new();
        for line in lines.lines() {
            let numbers: Vec<_> = line.split(":").last().unwrap().split("|").collect();
            let ticket_numbers: Vec<_> = numbers[0].trim().split(" ").filter(|x| !x.is_empty()).map(|x| u32::from_str(x).unwrap()).collect();
            let winning_numbers: Vec<_> = numbers[1].trim().split(" ").filter(|x| !x.is_empty()).map(|x| u32::from_str(x).unwrap()).collect();
            let card = Card { ticket_numbers, winning_numbers, owned: 1 };
            cards.push(card);
        }
        let mut total_cards = 0;
        for i in 0..cards.len() {
            let mut matches = 0;
            for ticket_number in &cards[i].ticket_numbers {
                if cards[i].winning_numbers.contains(&ticket_number) {
                    matches += 1;
                }
            }
            for j in 0..matches {
                let index = i + j + 1;
                if index < cards.len() {
                    let new_value = cards[index].owned + cards[i].owned;
                    cards[index].owned = new_value;
                }
            }


            total_cards += cards[i].owned;
        }


        println!("great success:{}", total_cards)
    }

    struct Card {
        ticket_numbers: Vec<u32>,
        winning_numbers: Vec<u32>,
        owned: u32,
    }
}