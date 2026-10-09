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
            // dbg!(self.remainder); // why is this valid? I thought the if let Some(...) above
            // moved remainder out of self?
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

#[derive(Debug)]
struct Person<'a> {
    name: Option<&'a str>,
    age: Option<u16>,
}

fn display_person<'a>(person: Person<'a>) {
    dbg!(person);
}

fn display_name(name: &str) {
    dbg!(name);
}

fn display_age(age: u16) {
    dbg!(age);
}

fn main() {
    let adophilus = Person {
        name: Some("Adophilus"),
        age: Some(22),
    };

    if let Some(name) = adophilus.name {
        // How is this possible? I thought once display_person returns, the person variable is dropped
        // (since display_person owned the person variable)
        display_person(adophilus);
        display_name(name);
        display_name(name);
    }

    // how Is this possible? I alreadpy gave ownership of the age variable to the display_age fn,
    // how can I give it out twice
    // if let Some(age) = adophilus.age {
    //     display_age(age);
    //     display_age(age);
    // }

    // are this posisbel bcos of copy semantics? Is this what copy semantics is?
}

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
