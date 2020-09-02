fn main() {
    println!(
        "{}",
        "lorem ipsum dolor sit amet"
            .split_whitespace()
            .map(|word| pig_latin(word).unwrap())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

/// Converts a string to pig latin. Returns `None` if `word` is empty.
///
/// The first consonant of each word is moved to the end of the word and “ay” is added.
/// Words that start with a vowel have “hay” added to the end instead.
///
/// For example, “first” becomes “irst-fay”, and “apple” becomes “apple-hay”.
fn pig_latin(word: &str) -> Option<String> {
    let mut word_chars = word.chars();

    let first = word_chars.next()?;

    Some(match word_chars.next() {
        _ if is_vowel(&first) => format!("{}-hay", word),
        None => format!("{}ay", first),
        Some(_) => format!("{}-{}ay", &word[first.len_utf8()..], first),
    })
}

fn is_vowel(c: &char) -> bool {
    "aeiou".contains(&c.to_lowercase().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pig_latin() {
        assert_eq!(pig_latin("apple").unwrap(), "apple-hay".to_string());
        assert_eq!(pig_latin("first").unwrap(), "irst-fay".to_string());
    }

    #[test]
    fn test_pig_latin_single_char() {
        assert_eq!(pig_latin("f").unwrap(), "fay".to_string());
        assert_eq!(pig_latin("क").unwrap(), "कay".to_string());
    }

    #[test]
    fn test_pig_latin_unicode() {
        assert_eq!(pig_latin("नमस्कार").unwrap(), "मस्कार-नay".to_string());
    }

    #[test]
    fn test_pig_latin_zero_length() {
        assert_eq!(pig_latin(""), None);
    }
}
