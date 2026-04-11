fn main() {
    let word1: &str = "amor";
    let word2: &str = "roma";
    let result: bool = is_anagram(word1, word2);

    println!("{word1} é anagrama de {word2}? {result}");
}

fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut word1: Vec<char> = word1.chars().collect();
    let mut word2: Vec<char> = word2.chars().collect();

    word1.sort();
    word2.sort();

    word1 == word2
}
