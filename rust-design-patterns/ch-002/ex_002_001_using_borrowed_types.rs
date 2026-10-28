/// Supposing we had used an owned type (e.g: `String`), this function would become less flexible
/// and then require callers to pass in a `String` or `&String`. But by leaving it as a `&str`,
/// callers can very easily just pass in a `&String` and have it auto deref to `&str`
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count == 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }

    false
}

pub fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious";

    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(curious));
}
