#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delim = &remainder[..next_delim];
                self.remainder = Some(&remainder[(next_delim + self.delimiter.len())..]);
                Some(until_delim)
            } else {
                let rest = remainder;
                self.remainder = None;
                Some(rest)
            }
        } else {
            None
        }
    }
}

fn main() {}

#[test]
fn test_that_it_works() {
    let haystack = "a b c d";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d"]);
}

#[test]
fn test_that_tail_empty_string_is_returned() {
    let haystack = "a b c d ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn test_that_consecutive_empty_strings_are_returned() {
    let haystack = "a b   c d ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "", "", "c", "d", ""]);
}

#[test]
fn test_that_consecutive_empty_tail_strings_are_returned() {
    let haystack = "a b c d  ";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "", ""]);
}
