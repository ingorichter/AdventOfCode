use crate::custom_error::AocError;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Eq)]
struct Hand {
    cards: Vec<u8>,
}

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
}

#[derive(Debug)]
struct HandAndBid {
    hand: Hand,
    bid: u32,
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
                'J' => hand.push(11),
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

    hand_and_bid.sort_by(|a, b| a.hand.cmp(&b.hand));

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
    use rand::rngs::mock::StepRng;
    use shuffle::{irs::Irs, shuffler::Shuffler};

    use super::*;

    const TEST_DATA_PART1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("6440", process(TEST_DATA_PART1)?);
        Ok(())
    }

    #[test]
    fn test_sort_single_pair() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![13, 13, 6, 7, 7],
        };
        let hand2 = Hand {
            cards: vec![13, 10, 11, 11, 10],
        };

        let mut hands = vec![&hand1, &hand2];
        // assert_eq!(hand1 < hand2, true);
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Greater);
        // assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Greater);

        // hands.sort();
        hands.sort_by(|a, b| a.cmp(&b));
        dbg!(&hands);
        assert_eq!(hands[0], &hand2);
        Ok(())
    }

    #[test]
    fn test_sort_misc() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![4, 4, 4, 4, 7],
        };
        let hand2 = Hand {
            cards: vec![2, 4, 5, 3, 4],
        };
        let hand3 = Hand {
            cards: vec![5, 5, 5, 5, 5],
        };

        let mut hands = vec![&hand1, &hand2, &hand3];

        hands.sort_by(|a, b| a.cmp(&b));
        dbg!(&hands);
        assert_eq!(hands[0], &hand2);
        Ok(())
    }

    #[test]
    fn test_compare_two_hands() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![13, 13, 6, 7, 7],
        };
        let hand2 = Hand {
            cards: vec![13, 10, 11, 11, 10],
        };

        assert_eq!(hand1.compare_positions(&hand2), std::cmp::Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_sort_all_five_card_hands() -> miette::Result<()> {
        let mut hands = vec![
            Hand {
                cards: vec![2, 2, 2, 2, 2],
            },
            Hand {
                cards: vec![3, 3, 3, 3, 3],
            },
            Hand {
                cards: vec![4, 4, 4, 4, 4],
            },
            Hand {
                cards: vec![5, 5, 5, 5, 5],
            },
            Hand {
                cards: vec![6, 6, 6, 6, 6],
            },
            Hand {
                cards: vec![7, 7, 7, 7, 7],
            },
            Hand {
                cards: vec![8, 8, 8, 8, 8],
            },
            Hand {
                cards: vec![9, 9, 9, 9, 9],
            },
            Hand {
                cards: vec![10, 10, 10, 10, 10],
            },
            Hand {
                cards: vec![11, 11, 11, 11, 11],
            },
            Hand {
                cards: vec![12, 12, 12, 12, 12],
            },
            Hand {
                cards: vec![13, 13, 13, 13, 13],
            },
            Hand {
                cards: vec![14, 14, 14, 14, 14],
            },
        ];

        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();
        let _ = irs.shuffle(&mut hands, &mut rng);

        hands.sort_by(|a, b| a.cmp(&b));
        assert_eq!(
            hands[0],
            Hand {
                cards: vec![2, 2, 2, 2, 2]
            }
        );
        assert_eq!(
            hands[hands.len() - 1],
            Hand {
                cards: vec![14, 14, 14, 14, 14]
            }
        );
        Ok(())
    }

    #[test]
    fn test_sort_all_various_hands() -> miette::Result<()> {
        let mut hands = vec![
            Hand {
                cards: vec![13, 10, 11, 11, 10],
            },
            Hand {
                cards: vec![3, 2, 10, 3, 13],
            },
            Hand {
                cards: vec![13, 13, 6, 7, 7],
            },
            Hand {
                cards: vec![12, 12, 12, 11, 14],
            },
            Hand {
                cards: vec![10, 5, 5, 11, 5],
            },
        ];

        hands.sort_by(|a, b| a.cmp(&b));

        assert_eq!(
            hands[0],
            Hand {
                cards: vec![3, 2, 10, 3, 13]
            }
        );
        assert_eq!(
            hands[hands.len() - 1],
            Hand {
                cards: vec![12, 12, 12, 11, 14]
            }
        );
        Ok(())
    }

    #[test]
    fn test_compare_hand() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![2, 3, 4],
        };
        let hand2 = Hand {
            cards: vec![4, 3, 2],
        };
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
        assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_compare_hand_equal() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![2, 3, 4],
        };
        let hand2 = Hand {
            cards: vec![2, 3, 4],
        };
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Equal);
        assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Equal);
        Ok(())
    }

    #[test]
    fn test_compare_fullhouse_with_threeofkind() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![2, 2, 2, 3, 3],
        };
        let hand2 = Hand {
            cards: vec![3, 3, 3, 3, 2],
        };
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
        assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_compare_fullhouse_with_four_of_kind() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![2, 2, 2, 3, 3],
        };
        let hand2 = Hand {
            cards: vec![3, 3, 3, 3, 2],
        };
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
        assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_compare_two_four_of_kinds() -> miette::Result<()> {
        let hand1 = Hand {
            cards: vec![2, 2, 2, 2, 3],
        };
        let hand2 = Hand {
            cards: vec![2, 3, 3, 3, 3],
        };
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Less);
        assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_compare_example_hands() -> miette::Result<()> {
        // KK677 and KTJJT
        let hand1 = Hand {
            cards: vec![13, 13, 6, 7, 7],
        };
        let hand2 = Hand {
            cards: vec![13, 10, 11, 11, 10],
        };
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Greater);
        assert_eq!(hand2.cmp(&hand1), std::cmp::Ordering::Less);
        Ok(())
    }
}
