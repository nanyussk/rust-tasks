
fn somas_reais(numbers: Vec<i32>) -> i32 {
    let mut soma: i32 = 0;
    for i in numbers{
        if i % 2 == 0{
            soma += i
        }
    }
    soma
}

fn main() {
    let numbers: Vec<i32> = vec![1, 3, 4, 2];

    let result: i32 = somas_reais(numbers);

    println!("A soma dos números reais é: {result}");
}
