
fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    
    let mut string_reversed: String = String::new();

    for c in chars {
        string_reversed.push(c);
    }

    string_reversed
}
fn main() {
    // Teste sua função com diferentes strings de entrada
    let input_string1 = String::from("hello");
    let reversed_string1 = reverse_string(&input_string1);
    println!("Original: {}", input_string1);
    println!("Reversed: {}", reversed_string1);
 
    let input_string2 = String::from("rust");
    let reversed_string2 = reverse_string(&input_string2);
    println!("Original: {}", input_string2);
    println!("Reversed: {}", reversed_string2);
}