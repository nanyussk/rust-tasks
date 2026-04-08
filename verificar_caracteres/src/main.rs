fn tem_caracteres_unicos(input: &str) -> bool {
    let mut char_list: Vec<_> = Vec::new();
    for i in input.chars() {
        if char_list.contains(&i) {
            return false
        } else {
            char_list.push(i);
        }
    }
    true
}

fn main() {
    let input: &str = "cateto";
    if tem_caracteres_unicos(input) {
        println!("A string possui todos os caracteres únicos."); 
    } else {
        println!("A string não possui todos os caracteres únicos."); 
    }
}
