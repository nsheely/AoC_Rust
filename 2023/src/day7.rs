// Day 7: Camel Cards
//
// Rank poker-like hands by strength and calculate total winnings.
// Part 1: Standard card rankings
// Part 2: Jokers (J) are wild cards that maximize hand strength

// Part 1 structures and functions
mod part1_impl {
    pub struct Hand {
        pub bid: u32,
        pub strength: u32,
    }

    impl Hand {
        #[inline]
        pub fn new(cards_str: &str, bid: u32) -> Self {
            let mut card_strength: u32 = 0;
            let mut cards: [u32; 13] = [0; 13];

            for (i, card) in (0_u32..).zip(cards_str.chars()) {
                let val = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 9,
                    'T' => 8,
                    n => n.to_digit(10).unwrap() - 2,
                };

                cards[val as usize] += 1;
                card_strength += val * (1 << ((4 - i) * 4));
            }

            let mut hand_type = 0;
            let mut pair_count = 0;
            let mut three_of_a_kind = false;

            for &count in &cards {
                match count {
                    5 => hand_type = 6, // Five of a kind
                    4 => hand_type = 5,
                    3 => {
                        if pair_count > 0 {
                            hand_type = 4;
                        } else {
                            three_of_a_kind = true;
                        }
                    },
                    2 => {
                        pair_count += 1;
                        if pair_count == 2 {
                            hand_type = 2;
                        } else if three_of_a_kind {
                            hand_type = 4;
                        } else {
                            hand_type = 1;
                        }
                    },
                    _ => {}
                }
            }

            if hand_type == 0 && three_of_a_kind {
                hand_type = 3;
            }

            let strength = (hand_type << 20) + card_strength;

            Hand { bid, strength }
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut hands: Vec<part1_impl::Hand> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cards_str = split.next().unwrap();
        let bid: u32 = split.next().unwrap().parse().unwrap();
        hands.push(part1_impl::Hand::new(cards_str, bid));
    }

    hands.sort_unstable_by_key(|hand| hand.strength);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
}

// Part 2 structures and functions
mod part2_impl {
    pub struct Hand {
        pub bid: u32,
        pub strength: u32,
    }

    impl Hand {
        #[inline]
        pub fn new(cards_str: &str, bid: u32) -> Self {
            let mut strength: u32 = 0;
            let mut jokers = 0;
            let mut cards: [u32; 13] = [0; 13];

            cards_str.chars().enumerate().for_each(|(i, card)| {
                let val = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => { jokers += 1; 0 },
                    'T' => 9,
                    n => n.to_digit(10).unwrap() - 1,
                };

                if val != 0 {
                    cards[val as usize] += 1;
                }

                strength |= val << ((4 - i) * 4);
            });

            cards.sort_unstable();

            let hand_type = match cards[12] + jokers {
                5 => 6,
                4 => 5,
                3 if cards[11] == 2 => 4,
                3 => 3,
                2 if cards[11] == 2 => 2,
                2 => 1,
                _ => 0,
            };

            strength |= hand_type << 20;

            Hand { bid, strength }
        }
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let mut hands: Vec<part2_impl::Hand> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cards_str = split.next().unwrap();
        let bid: u32 = split.next().unwrap().parse().unwrap();
        hands.push(part2_impl::Hand::new(cards_str, bid));
    }

    hands.sort_unstable_by_key(|hand| hand.strength);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
}
