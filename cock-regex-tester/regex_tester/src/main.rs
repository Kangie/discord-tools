// Copyright (C) 2025 Matt Jolly <kangie@gentoo.org>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use lazy_static::lazy_static;
use regex::Regex;
use std::io::{stdin, stdout, Write};

// Programmatically generate a regex to match prefixes & suffixes
// How this works:
// Prefix + suffix
// 1. Try to match on compound words first
// 2. Then try to match on only prefixes
// We should not match on e.g. `cockpit` because that's innocuous.
fn generate_offensive_regex(prefixes: &[&str], suffixes: &[&str]) -> String {
    let mut regex_str = r"\b(".to_string();
    for (i, prefix) in prefixes.iter().enumerate() {
        if i > 0 {
            regex_str.push_str("|");
        }
        regex_str.push_str(prefix);
    }
    regex_str.push_str(r")("); // Start of suffix group
    for (i, suffix) in suffixes.iter().enumerate() {
        if i > 0 {
            regex_str.push_str("|");
        }
        regex_str.push_str(suffix);
    }
    regex_str.push_str(r")s?|\b("); // End of suffix group, allow 's' after suffixes
    for (i, prefix) in prefixes.iter().enumerate() {
        if i > 0 {
            regex_str.push_str("|");
        }
        regex_str.push_str(prefix);
    }
    regex_str.push_str(r")s?\b");
    regex_str
}
// We only want to define this regex once, so use `lazy_static` and reference
// if run from the command line or invoked using `cargo test`
lazy_static! {
    static ref OFFENSIVE_REGEX: Regex = {
        let prefixes = ["[c|k](ock|0ck|\\*ck|\\*\\*k)"];
        let suffixes = [
            "face",
            "head",
            "lover",
            "tease",
            r"blast(er|ing)?",
            r"block(er|ing)?",
            r"master",
            r"munch(er|ing)?",
            r"suck(er|ing)",
        ];
        let regex_str = generate_offensive_regex(&prefixes, &suffixes);
        Regex::new(&regex_str).unwrap()
    };
}

fn is_offensive(text: &str) -> bool {
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in words {
        let word_lower = word.to_lowercase();
        if let Some(captures) = OFFENSIVE_REGEX.captures(&word_lower) {
            println!("Matched offensive word: {}", word_lower);
            // Print all capture groups
            for i in 0..=captures.len() {
                if let Some(capture) = captures.get(i) {
                    println!("  Capture {}: {}", i, capture.as_str());
                }
            }
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offensive_regex() {

        let bad_basic_words = [
            "c0ck",
            "c0cks",
            "cock-blocking",
            "cock",
            "cockblasting",
            "cockblocking",
            "cockface",
            "cockmuncher",
            "cocks",
            "cocksucker",
            "cocksuckers",
            "c*ck"
        ];

        for word in bad_basic_words {
            assert_eq!(
                is_offensive(word),
                true,
                "Test failed: expected \"{}\" to fail validation.",
                word
            );
        }

        let bad_phrases = [
            "He is a cock",
            "He is cocksucker",
            "This is a cocksucking",
            "that c*ckhead",
            "Say 'cocksucker' one more time!",
            "Go ahead, make my cocktease.",
            "You talkin' to my cockhead?",
            "I'll be cockblocked.",
            "Game over, cockmaster.",
            "My precious cocksucker.",
            "Cocktease me once, shame on you. Cocktease me twice...",
            "Game over, c**kmaster.",
        ];

        for phrase in bad_phrases {
            assert_eq!(
                is_offensive(phrase),
                true,
                "Test failed: expected phrase \"{}\" to fail validation.",
                phrase
            );
        }

        // Negative cases (should be legitimate)

        let good_basic_words = [
            "cockaded",
            "cockings",
            "cockpit",
            "cocktail",
            "cocky",
            "locomotive",
        ];

        for word in good_basic_words {
            assert_eq!(
                is_offensive(word),
                false,
                "Test failed: expected \"{}\" to pass validation.",
                word
            );
        }

        let good_phrases = [
            "This is a cockaded bird",
            "I hit the shuttlecock hard",
            "Look at that cockchafer!",
            "The quick brown fox jumps over the lazy cockatoo.",
            "I'm sure the bartender can make a mean strawberry cocktail.",
            "The proud cockerel strutted around the farmyard.",
            "He woke up at cockcrow to the sound of roosters.",
            "The historical documentary discussed the controversial practice of cockfighting.",
            "She was cocksure that her team would win the championship.",
        ];

        for phrase in good_phrases {
            assert_eq!(
                is_offensive(phrase),
                false,
                "Test failed: expected phrase \"{}\" to pass validation.",
                phrase
            );
        }

        // "cock-a-doodle-doo" fails but should pass. We need negative lookaheads to special case that which the crate does not contain.
        let edge_cases = [
            "I am cocking a gun",
        ];

        for edge_case in edge_cases {
            assert_eq!(
                is_offensive(edge_case),
                false,
                "Test failed: expected edge case \"{}\" to pass validation.",
                edge_case
            );
        }

    }
}

fn main() {
    println!("Kangie's dodgy regex tester");
    println!("Offensive regex: {}", OFFENSIVE_REGEX.as_str());
    println!("");
    loop {
        print!("Enter text: ");
        stdout().flush().unwrap(); // Flush the output buffer to ensure the prompt is displayed

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim(); // Trim whitespace from the input

        if input == "exit" {
            break;
        }

        if is_offensive(input) {
            println!("The text contains offensive words.");
        } else {
            println!("The text is clean.");
        }
    }
}
