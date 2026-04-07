
fn main() {
    let number: i32 = 5;

    println!("Tabuada de {number}:");
    for i in 1..11 {
        let result: i32 = number * i;
        println!("{number}x{i} = {result}");
    }
}
