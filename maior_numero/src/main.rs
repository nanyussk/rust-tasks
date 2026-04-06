
fn maior_numero() {
    let num_lista: Vec<i32> = vec![1, 10, 3, 26, 88, 9, 55];
    let mut maior: i32 = 0;

    for i in num_lista {
        if i > maior {
            maior = i;
        }
    }
    println!("O maior numero da lista é: {maior}")

}

fn main() {
    maior_numero();
}
