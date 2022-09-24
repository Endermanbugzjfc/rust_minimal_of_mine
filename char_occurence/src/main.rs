use colored::*;
use std::{collections::HashMap, env};

fn main() {
    let mut args = env::args();
    args.next();
    let string = args.collect::<String>();

    let occurences = get_occurences(&string);
    occurences.iter().count();
}

fn get_occurences(string: &str) -> HashMap<char, u8> {
    let mut occurences: HashMap<char, u8> = HashMap::new();
    for c in string.chars() {
        let entry = occurences.entry(c).or_insert(0);
        *entry += 1
    }

    occurences
}

fn get_char_rgb_gap(unique_char: usize) -> Option<[u8; 3]> {
    if unique_char > 256 * 3 {
        None
    } else {
        let rgb_len = {
            let r = unique_char % 3;
            let g = unique_char / 3;
            let b = unique_char / 9;

            [r, g, b]
        };
        let mut rgb_gap = [0u8; 3];

        for i in 0..3 {
            // "Inflate" RGB to fill u8.
            let c = rgb_len[i];
            if c != 0 {
                rgb_gap[i] += 255 / c as u8
            };
        }

        Some(rgb_gap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_occurences() {
        assert_eq!(get_occurences("").iter().count(), 0);
    }

    #[test]
    fn unique_occurences_ascii() {
        assert_eq!(get_occurences("Mienraft").values().sum::<u8>(), 8);
        // pls vote 4 https://gist.github.com/PEMapModder/cb60bd61b5f5d08ae3dc81781feb8449?permalink_comment_id=3900366#gistcomment-3900366
    }

    #[test]
    fn occurences() {
        assert_eq!(get_occurences("FloCcINAUCINIHiLIpILIFicAtion")[&'I'], 6);
    }

    #[test]
    fn unique_occurences_chinese() {
        assert_eq!(get_occurences("又双叒叕").values().sum::<u8>(), 4);
    }

    #[test]
    fn occurences_chinese() {
        assert_eq!(get_occurences("习习白習卒翠")[&'习'], 2);
    }

    #[test]
    fn red_rgb_gap() {
        assert_eq!(get_char_rgb_gap(1).expect("1 <= 256 * 3"), [255, 0, 0]);
    }

    #[test]
    fn middle_char_rgb_gap() {
        assert_eq!(
            get_char_rgb_gap(128 * 3).expect("128 * 3 <= 256 * 3"),
            [0, 1, 6]
        );
    }

    #[test]
    fn too_many_unique_char_rgb_gap() {
        assert_eq!(get_char_rgb_gap(256 * 3 + 1), None);
    }
}
