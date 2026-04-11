fn main() {
    let numbers: [i32; 3] = [121, -121, 10];

    for i in numbers {
        let result = is_polindromo(i);
        println!("Número {i} é políndromo: {result}");
    }
}

fn is_polindromo(number: i32) -> bool {
    let original = number.to_string();
    let reversed: String = original.chars().rev().collect();

    original == reversed
}