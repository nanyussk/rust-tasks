fn main() {
    let var_a: i32 = 15;
    let var_b: i32 = 40;

    let mut divisor: i32 = if var_a < var_b { var_a } else { var_b };

    while var_a % divisor != 0 || var_b % divisor != 0 {
        divisor -= 1;
    }

    print!("O maior divisor comum entre {} e {} é {}", var_a, var_b, divisor);
}