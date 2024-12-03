use std::cmp::Ordering;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use fancy_regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,      // ([2-9TJQKA])\1
    TwoPair,      // ([2-9TJQKA])\1.?([2-9TJQKA])\2
    ThreeOfAKind, // ([2-9TJQKA])\1{2}
    FullHouse,    // ([2-9TJQKA])\1{2}([2-9TJQKA])\2 | ([2-9TJQKA])\1{1}([2-9TJQKA])\2{2} 
    FourOfAKind,  // ([2-9TJQKA])\1{3}
    FiveOfAKind,  // ([2-9TJQKA])\1{4}
}

fn compare_card(card1: Vec<char>, card2: Vec<char>) -> Ordering {
    for i in 0..5 {
        let card1_numeric = card1[i].is_numeric();
        let card2_numeric = card2[i].is_numeric();
        let ordering = if card1_numeric { 
          if card2_numeric {
            card1[i].cmp(&card2[i])
          } else {
            Ordering::Less
          } 
        } else if card2_numeric {
            Ordering::Greater
        } else {
            match (card1[i], card2[i]) {
              ('A', x) if x != 'A' => Ordering::Greater,
              ('K', 'A') => Ordering::Less,
              ('K', x) if x != 'K' => Ordering::Greater,
              ('Q', 'A') => Ordering::Less,
              ('Q', 'K') => Ordering::Less,
              ('Q', x) if x != 'Q' => Ordering::Greater,
              ('J', 'A') => Ordering::Less,
              ('J', 'K') => Ordering::Less,
              ('J', 'Q') => Ordering::Less,
              ('J', x) if x != 'J' => Ordering::Greater,
              ('T', 'A') => Ordering::Less,
              ('T', 'K') => Ordering::Less,
              ('T', 'Q') => Ordering::Less,
              ('T', 'J') => Ordering::Less,              
              ('T', x) if x != 'T' => Ordering::Greater,
              _ => Ordering::Equal
            }
        };
        if ordering != Ordering::Equal {
            return ordering
        }
        
    } 
    Ordering::Equal
}

fn main() {
    let entry_re = regex::Regex::new(r"^([2-9TJQKA]+) (\d+)").unwrap();

    let hand_type_re_list = vec![
        (Regex::new(r"([2-9TJQKA])\1{4}").unwrap(), HandType::FiveOfAKind),
        (Regex::new(r"([2-9TJQKA])\1{3}").unwrap(), HandType::FourOfAKind),
        (Regex::new(r"([2-9TJQKA])\1{2}([2-9TJQKA])\2").unwrap(), HandType::FullHouse),
        (Regex::new(r"([2-9TJQKA])\1{1}([2-9TJQKA])\2{2}").unwrap(), HandType::FullHouse),
        (Regex::new(r"([2-9TJQKA])\1{2}").unwrap(), HandType::ThreeOfAKind),
        (Regex::new(r"([2-9TJQKA])\1.?([2-9TJQKA])\2").unwrap(), HandType::TwoPair),
        (Regex::new(r"([2-9TJQKA])\1").unwrap(), HandType::OnePair)
    ];

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();    

    let mut hands:Vec<(String, HandType, usize)> = BufReader::new(file).lines().map(|line| {
        let line_str = &line.unwrap();


        let (hand, bid) = entry_re.captures_iter(&line_str)
            .map(|hand_bid_cap| {
                (hand_bid_cap[1].to_string(), hand_bid_cap[2].parse::<usize>().unwrap())
            }).next().unwrap();

        let mut hand_chars: Vec<char> = hand.chars().collect();
        hand_chars.sort();
        let sorted_hand: String = hand_chars.into_iter().collect();

        let hand_type = hand_type_re_list.iter()
            .filter(|&tuple| { tuple.0.is_match(&sorted_hand).unwrap()})
            .map(|tuple| tuple.1)
            .next().unwrap_or(HandType::HighCard);

        (hand, hand_type, bid)
    }).collect();
        
    hands.sort_by(|a,b| {
       if a.1 < b.1 {
        Ordering::Less
       } else if a.1 > b.1 {
        Ordering::Greater
       } else {
        compare_card(a.0.chars().collect(), b.0.chars().collect())
       }
    });

    for hand in &hands {
        println!("{:?}", hand);
    }

    let total = hands.iter().enumerate().fold(0, |accum ,tuple| {
      accum + (tuple.0 + 1) * tuple.1.2 
    });

    println!("Total = {}", total);
}