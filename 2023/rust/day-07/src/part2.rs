use crate::custom_error::AocError;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Default, Clone, Eq)]
struct Hand {
    cards: Vec<u8>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    Random = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

const JOKER:u8 = 1;

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // println!("Self: {:?}", self.cards);
        // println!("Other: {:?}", other.cards);
    
        // identical
        if self.cards.cmp(&other.cards) == std::cmp::Ordering::Equal {
            return std::cmp::Ordering::Equal;
        }

        // throw them in a set and count how many are left
        let mut self_set = std::collections::HashSet::new();
        for card in &self.cards {
            self_set.insert(card);
        }

        let mut other_set = std::collections::HashSet::new();
        for card in &other.cards {
            other_set.insert(card);
        }

        let self_distinct_card_count = self_set.len();
        let other_distinct_card_count = other_set.len();

        match self_distinct_card_count.cmp(&other_distinct_card_count) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                if self_distinct_card_count == 2 || self_distinct_card_count == 3 {
                    let mut self_cards: HashMap<u8, u8> = HashMap::new();
                    for card in &self.cards {
                        let count = self_cards.entry(*card).or_insert(0);
                        *count += 1;
                    }
    
                    let mut other_cards: HashMap<u8, u8> = HashMap::new();
                    for card in &other.cards {
                        let count = other_cards.entry(*card).or_insert(0);
                        *count += 1;
                    }
    
                    let max_card_num_self = *self_cards.values().max().unwrap();
                    let max_card_num_other = *other_cards.values().max().unwrap();
    
                    match max_card_num_self.cmp(&max_card_num_other) {
                        std::cmp::Ordering::Equal => self.compare_positions(other),
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater
                    }
                } else {
                    self.compare_positions(other)
                }
            }
        }
    }
}

impl Hand {
    fn compare_positions(&self, other: &Hand) -> std::cmp::Ordering {
        for i in 0..self.cards.len() {
            match self.cards[i].cmp(&other.cards[i]) {
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            
            }
        }
        std::cmp::Ordering::Equal
    }

    fn maximize_hand(&self) -> Hand {
        if self.cards.contains(&JOKER) {
            let mut self_cards: HashMap<u8, u8> = HashMap::new();
            for card in &self.cards {
                let count = self_cards.entry(*card).or_insert(0);
                *count += 1;
            }

            // card that are we replacing J with
            // dbg!(&self_cards);
            let max_entry = self_cards.iter().max_by(|a, b| {
                // Compare values first
                let key_cmp = a.1.cmp(b.1);
                if key_cmp == std::cmp::Ordering::Equal {
                    // If values are equal, compare keys
                    a.0.cmp(b.0)
                } else {
                    key_cmp
                }
            });

            let replaced_value = max_entry.unwrap().0;

            return Hand {
                cards: self.cards.iter().map(|c| {
                    if *c == JOKER {
                        *replaced_value
                    } else {
                        *c
                    }
                }).collect()
            }
        }

        self.clone()
    }

    fn get_type(&self) -> HandType {
        let mut cards: HashMap<u8, u8> = HashMap::new();
        for card in &self.cards {
            let count = cards.entry(*card).or_insert(0);
            *count += 1;
        }

        match cards.len() {
            2 => {
                match cards.values().max().unwrap() {
                    3 => HandType::FullHouse,
                    _ => HandType::FourOfAKind,
                }
            }
            3 => {
                match cards.values().max().unwrap() {
                    2 => HandType::TwoPair,
                    _ => HandType::ThreeOfAKind,
                }
            }
            4 => HandType::OnePair,
            _ => HandType::Random
        }
    }
}

#[derive(Debug)]
struct HandAndBid {
    hand: Hand,
    bid: u32,
}

fn compare_with_maximized_hand(a: &HandAndBid, b: &HandAndBid) -> Ordering {
    let maximized_a = a.hand.maximize_hand();
    let maximized_b = b.hand.maximize_hand();

    // println!("Hand A: {:?} => {:?} ({:?})", a.hand, maximized_a, maximized_a.get_type());
    // println!("Hand B: {:?} => {:?} ({:?})", b.hand, maximized_b, maximized_b.get_type());

    if maximized_a.get_type() == maximized_b.get_type() {
        // println!("Both hands have the same type");

        if a.hand.cmp(&b.hand) == Ordering::Equal {
            // println!("Both hands are identical");
            return Ordering::Equal;
        }

        return a.hand.cards[0].cmp(&b.hand.cards[0]);
        // for (i, hand_avalue) in a.hand.cards.iter().enumerate() {
        //     return hand_avalue.cmp(&b.hand.cards[i]);
        // }
    }

    if maximized_a.get_type() > maximized_b.get_type() {
        return Ordering::Greater;
    }

    return Ordering::Less;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut hand_and_bid: Vec<HandAndBid> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            // let hand = split.next().unwrap().chars().collect();
            let mut hand: Vec<u8> = vec![];
            split.next().unwrap().chars().for_each(|c| match c {
                '2' => hand.push(2),
                '3' => hand.push(3),
                '4' => hand.push(4),
                '5' => hand.push(5),
                '6' => hand.push(6),
                '7' => hand.push(7),
                '8' => hand.push(8),
                '9' => hand.push(9),
                'T' => hand.push(10),
                'J' => hand.push(1),
                'Q' => hand.push(12),
                'K' => hand.push(13),
                'A' => hand.push(14),
                _ => panic!("Invalid card"),
            });

            let bid = split.next().unwrap().parse::<u32>().unwrap();
            HandAndBid {
                hand: Hand { cards: hand },
                bid,
            }
        })
        .collect();

    // let mut maximized_hand_and_bid: Vec<HandAndBid> = hand_and_bid.iter().map(|vab| {
    //     let maximized_hand = vab.hand.maximize_hand();
    //     println!("Hand {:?} is maximized {:?}", vab.hand, maximized_hand);
    //     HandAndBid {
    //         hand: maximized_hand,
    //         bid: vab.bid
    //     }
    // }).collect();

    // maximized_hand_and_bid.sort_by(|a, b| a.hand.cmp(&b.hand));
    hand_and_bid.sort_by(|a, b| {
        compare_with_maximized_hand(a, b)
    });

    // dbg!(&hand_and_bid);
    // hand_and_bid.iter().for_each(|f| println!("{:?}", f));

    let mut index = 0;
    let result = hand_and_bid
        .iter()
        .map(|h| {
            index += 1;
            h.bid * index
        })
        .sum::<u32>();

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART2: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    
    // sort hands decending by value of hand
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("5905", process(TEST_DATA_PART2)?);
        Ok(())
    }

    #[test]
    fn test_maximize_hand() -> miette::Result<()> {
        let hand = Hand {
            cards: vec![10, 5, 5, JOKER, 5],
        };

        let expected_hand = Hand {
            cards: vec![10, 5, 5, 5, 5]
        };

        // turn into four of a kind
        let maximized_hand = hand.maximize_hand();
        assert_eq!(maximized_hand.cmp(&expected_hand), std::cmp::Ordering::Equal);

        let hand2 = Hand {
            cards: vec![12, JOKER, JOKER, 12, 2]
        };

        let expected_hand2 = Hand {
            cards: vec![12, 12, 12, 12, 2]
        };

        let maximized_hand2 = hand2.maximize_hand();
        assert_eq!(maximized_hand2.cmp(&expected_hand2), std::cmp::Ordering::Equal);

        Ok(())
    }

    #[test]
    fn test_hand_types() -> miette::Result<()> {
        let hand = Hand {
            cards: vec![10, 5, 5, 5, 5],
        };

        let hand_type = hand.get_type();
        assert_eq!(HandType::FourOfAKind, hand_type);

        let hand = Hand {
            cards: vec![10, 5, 5, 5, 10],
        };

        assert_eq!(HandType::FullHouse, hand.get_type());

        let hand = Hand {
            cards: vec![10, 5, 3, 5, 10],
        };

        assert_eq!(HandType::TwoPair, hand.get_type());

        let hand = Hand {
            cards: vec![10, 2, 3, 4, 10],
        };

        assert_eq!(HandType::OnePair, hand.get_type());

        let hand = Hand {
            cards: vec![10, 10, 3, 4, 10],
        };

        assert_eq!(HandType::ThreeOfAKind, hand.get_type());

        Ok(())
    }

    #[test]
    fn test_compare_with_maximized_hand() -> miette::Result<()> {
        let hand1 = HandAndBid {
            hand: Hand { cards: vec![12, JOKER, JOKER, 12, 2] },
            bid: 2,
        };

        let hand2 = HandAndBid {
            hand: Hand { cards: vec![JOKER, 13, 13, 13, 2] },
            bid: 2,
        };

        assert_eq!(compare_with_maximized_hand(&hand1, &hand2), Ordering::Greater);

        Ok(())
    }

    #[test]
    fn test_sort_with_maximized_hand() -> miette::Result<()> {
        let hand1 = HandAndBid {
            hand: Hand { cards: vec![10, 5, 5, JOKER, 5] },
            bid: 3,
        };

        let hand2 = HandAndBid {
            hand: Hand { cards: vec![13, 10, JOKER, JOKER, 10] },
            bid: 5,
        };

        let hand3 = HandAndBid {
            hand: Hand { cards: vec![12, 12, 12, JOKER, 14] },
            bid: 4,
        };

        assert_eq!(compare_with_maximized_hand(&hand1, &hand2), std::cmp::Ordering::Less);
        assert_eq!(compare_with_maximized_hand(&hand2, &hand1), std::cmp::Ordering::Greater);
        assert_eq!(compare_with_maximized_hand(&hand1, &hand3), std::cmp::Ordering::Less);
        assert_eq!(compare_with_maximized_hand(&hand2, &hand3), std::cmp::Ordering::Greater);

        // T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5
        let mut hand_and_bid: Vec<HandAndBid> = vec![hand1, hand2, hand3];

        hand_and_bid.sort_by(compare_with_maximized_hand);

        assert_eq!(hand_and_bid[0].bid, 3);
        assert_eq!(hand_and_bid[1].bid, 4);
        assert_eq!(hand_and_bid[2].bid, 5);

        Ok(())
    }
}