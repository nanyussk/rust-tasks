
fn count(num: &i32) {
    println!("Contando de 1 a {}:", num);
    for i in 1..num+1{
        println!("Número {}", i)
    }
}
fn count_down(num: &i32) {
    println!("Contando de {} a 1:", num);

    let mut current = *num;
    while current >= 1 {
        println!("Número {}", current);
        current -= 1;
    }
}

fn main() {
    let number: i32 = 10;
    count(&number);
    count_down(&number);
}
