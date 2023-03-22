use std::iter::Iterator;
use std::ops::Range;
use std::string::String;

const EMPTY: CapitalizeError = CapitalizeError::Empty("No sentence were given!");
const IGNORED_WORDS: [&'static str; 18] = ["a", "and", "as", "at", "but", "by", "for", "if", "in", "nor", "of", "off", "on", "or", "the", "to", "up", "vs"];

#[derive(PartialEq, Debug)]
pub enum CapitalizeError {
    Empty(&'static str)
}

#[derive(PartialEq, Debug)]
pub struct CapitalizedSentence {
    pub sentence: String,
}

impl CapitalizedSentence {
    fn new(value: &Vec<String>) -> CapitalizedSentence {
        let sentence_vec: Vec<String> = value
            .iter()
            .map(|w| {
                match IGNORED_WORDS.contains(&w.as_str()) {
                    true => w.to_string(),
                    false => w[0..1].to_uppercase() + &w[1..]
                }
            }).collect();

        let mut sentence = sentence_vec.join(" ");

        if !sentence.chars().next().unwrap().is_alphabetic() {
            Self::replace_letter_range(&mut sentence, 2..3);
        }
        Self::replace_letter_range(&mut sentence, 0..1);
        CapitalizedSentence { sentence }
    }

    fn replace_letter_range(sentence: &mut String, range: Range<usize>) {
        sentence.replace_range(range.clone(), &*sentence[range].to_uppercase());
    }
}

pub fn capitalize(sentence: &Option<Vec<String>>, reverse: bool) {
    match cap(sentence) {
        Ok(s) => {
            let sentence  = s.sentence;
            match reverse {
                true => println!("{}", sentence.chars().rev().collect::<String>()),
                false => println!("{}", sentence)
            }
        },
        Err(err) => panic!("{:?}",err)
    }
}

fn cap(sentence: &Option<Vec<String>>) -> Result<CapitalizedSentence, CapitalizeError>  {
    let sentence = sentence.clone().ok_or(EMPTY)?;
    Ok(CapitalizedSentence::new(sentence.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_input() {
        let mut vec = Vec::new();
        vec.push("a".to_string());
        vec.push("river".to_string());
        vec.push("and".to_string());
        vec.push("a".to_string());
        vec.push("city".to_string());
        assert_eq!(
            cap(&Some(vec)),
            Ok(CapitalizedSentence { sentence: "A River and a City".to_string() })
        );
    }

    #[test]
    fn capitalize_input_with_number() {
        let mut vec = Vec::new();
        vec.push("1".to_string());
        vec.push("a".to_string());
        vec.push("river".to_string());
        vec.push("and".to_string());
        vec.push("a".to_string());
        vec.push("city".to_string());
        assert_eq!(
            cap(&Some(vec)),
            Ok(CapitalizedSentence { sentence: "1 A River and a City".to_string() })
        );
    }

    #[test]
    fn capitalize_input_with_symbol() {
        let mut vec = Vec::new();
        vec.push("#".to_string());
        vec.push("a".to_string());
        vec.push("river".to_string());
        vec.push("and".to_string());
        vec.push("a".to_string());
        vec.push("city".to_string());
        assert_eq!(
            cap(&Some(vec)),
            Ok(CapitalizedSentence { sentence: "# A River and a City".to_string() })
        );
    }

    #[test]
    fn capitalize_empty_input() {
        assert_eq!(
            cap(&None),
            Err(EMPTY)
        );
    }
}
