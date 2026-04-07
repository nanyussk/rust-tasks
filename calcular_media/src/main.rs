fn calcular_media(notas: Vec<f32>) -> f32 {
    let mut soma: f32 = 0.0;
    for i in &notas {
        soma += i;
    }
    soma / notas.len() as f32
}

fn main() {
    let aluno_notas = vec![6.5, 7.0, 8.0, 10.0];
    let resultado = calcular_media(aluno_notas);

    println!("A media do aluno é: {resultado:.1}")
}
