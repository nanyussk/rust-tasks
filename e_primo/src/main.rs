
fn e_primo(numero: i32) -> bool {
    if numero <= 1 {
        return false;
    } else {
        let limite: i32 = (numero as f64).sqrt().ceil() as i32;
        for i in 2..=limite {
            if numero % i == 0 && numero != i {
                return false;
            }
        }
    }
    true
}

fn main() {
    let numero: i32 = 53;
    let resultado = e_primo(numero);
    print!("O número {numero} é primo? {resultado}");
}
