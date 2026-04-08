fn e_permutacao(string_1: &str, string_2: &str) -> bool {
    if string_1.len() != string_2.len() {
        return false;
    } else {
        let mut chars1: Vec<_> = string_1.chars().collect();
        let mut chars2: Vec<_> = string_2.chars().collect();

        chars1.sort();
        chars2.sort();

        chars1 == chars2
    }
}

fn main() {
    let string_1: &str = "amor";
    let string_2: &str = "roma";

    if e_permutacao(string_1, string_2) {
        println!("É permutação.")
    } else {
        println!("Não é permutação.")
    }
}
