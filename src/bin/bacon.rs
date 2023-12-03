use std::iter::FromIterator;
use bimap::BiMap;

static TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
            nisi ut aliquip ex ea commodo consequat. Quis autem vel eum iure \
            reprehenderit qui in ea voluptate";

struct BaconEncodeDecoder {
    bi_map: BiMap<char, u8>
}

impl BaconEncodeDecoder {
    fn new(alphabet: [char;30]) -> BaconEncodeDecoder {
        BaconEncodeDecoder {
            bi_map: (1..alphabet.len() + 1).into_iter()
                .fold(
                    BiMap::new(),
                    |mut it, i| {
                        it.insert(alphabet[i - 1].to_ascii_uppercase(), i as u8);
                        it
                    }
                )
        }
    }

    fn default() -> BaconEncodeDecoder {
        BaconEncodeDecoder::new([
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            '.', '?', ',', '\''
        ])
    }

    fn encode(&mut self, message: &str) -> Option<String> {
        let mut text_iter = TEXT.chars().into_iter();

        let mut result: Vec<char> = Vec::new();

        for mut message_char in message.chars() {
            message_char.make_ascii_uppercase();
            if let Some(value) = self.bi_map.get_by_left(&message_char).or( Some(&0b00000)) {
                let mut mut_value = *value;
                for i in (0..5).rev() {
                    let mut found = false;
                    while !found {
                        if let Some(mut text_char) = text_iter.next() {
                            text_char.make_ascii_uppercase();
                            if text_char.is_ascii_alphabetic() {
                                found = true;
                                let power_of_two = 2u8.pow(i);
                                if mut_value >= power_of_two {
                                    result.push(text_char.to_ascii_uppercase());
                                    mut_value -= power_of_two;
                                } else {
                                    result.push(text_char.to_ascii_lowercase());
                                }
                            } else {
                                result.push(text_char)
                            }
                        } else {
                            return None;
                        }
                    }
                }
            } else {
                result.push(message_char)
            }
        }

        while let Some(text_char) = text_iter.next() {
            result.push(text_char.to_ascii_lowercase())
        }

        Some(String::from_iter(result.iter()))
    }

    fn decode(&mut self, text: &str) -> String {
        let mut result: Vec<char> = Vec::new();

        let mut alpha_text_iter = text.chars().filter(
            |it| it.is_ascii_alphabetic()
        );

        let mut char_count: u8 = 0;
        let mut index: u8 = 0;
        while let Some(alpha_text) = alpha_text_iter.next() {
            index <<= 1;
            char_count += 1;
            if alpha_text.is_ascii_uppercase() {
                index += 1;
            }
            // if index == 0b11111 {
            //    break;
            //}
            if char_count == 5 {
                result.push(*self.bi_map.get_by_right(&index).unwrap_or(&' '));
                char_count = 0;
                index = 0;
            }
        }

        String::from_iter(result.iter())
    }
}

fn main() {
    let message = "By Grabthar's Hammer, you shall be avenged.";

    let mut encode_decoder = BaconEncodeDecoder::default();

    let encoded = encode_decoder.encode(message).unwrap_or(String::from("error"));

    println!("{}", encoded);
    println!("{}", encode_decoder.decode(encoded.as_str()));
    println!("{}", encode_decoder.decode("this Is a tEst. thIS is A teST tHIs").as_str());
}
