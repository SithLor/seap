pub fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
pub fn does_have_vowels(s: &str) -> bool {
    s.chars().any(|c| is_vowel(c))
}
pub fn count_lowercase_vowels(s: &str) -> usize {
    s.chars().filter(|c| is_vowel(*c)).count()
}
pub fn count_uppercase_vowels(s: &str) -> usize {
    s.chars().filter(|c| is_vowel(*c)).count()
}
pub const VOWELS_LOWERCASE: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
pub const VOWELS_UPPERCASE: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
pub enum VowelsUppercase {
    A,
    E,
    I,
    O,
    U,
}
pub enum VowelsUndercase {
    a,
    e,
    i,
    o,
    u,
}
