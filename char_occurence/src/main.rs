use indoc::indoc;
use std::{collections::HashMap, env};

static SUPERSCRIPT_NUMBERS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

fn main() {
    let mut args = env::args();
    args.next();
    let string = args.collect::<String>();

    if string.chars().count() == 0 {
        eprintln!(indoc! {r#"
            Example:
            $ char_occurence "hello"

            Output:
            h¹e¹l²l²o¹
        "#});
        return;
    }

    let occurences = get_occurences(&string);

    println!(
        "{}",
        string
            .chars()
            .map(|c| {
                let mut string = c.to_string();
                string.push(SUPERSCRIPT_NUMBERS[occurences[&c]]);

                string
            })
            .collect::<String>()
    );
}

fn get_occurences(string: &str) -> HashMap<char, usize> {
    let mut occurences: HashMap<char, usize> = HashMap::new();
    for c in string.chars() {
        let entry = occurences.entry(c).or_insert(0);
        *entry += 1
    }

    occurences
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
        assert_eq!(get_occurences("Mienraft").values().sum::<usize>(), 8);
        // pls vote 4 https://gist.github.com/PEMapModder/cb60bd61b5f5d08ae3dc81781feb8449?permalink_comment_id=3900366#gistcomment-3900366
    }

    #[test]
    fn occurences() {
        assert_eq!(get_occurences("FloCcINAUCINIHiLIpILIFicAtion")[&'I'], 6);
    }

    #[test]
    fn unique_occurences_chinese() {
        assert_eq!(get_occurences("又双叒叕").values().sum::<usize>(), 4);
    }

    #[test]
    fn occurences_chinese() {
        assert_eq!(get_occurences("习习白習卒翠")[&'习'], 2);
    }
}
