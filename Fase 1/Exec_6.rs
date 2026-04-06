// Declare uma constante PI com o valor 3.1415 e use-a para calcular a área de um círculo.
use std::io;

const PI: f64 = 3.1415;

fn main(){
    // Solicitando valor do raio ao usuário
    println!("Informe a área do círculo: ");
    let mut raio = String::new();
    io::stdin().read_line(&mut raio).expect("Falha ao ler.");

    // Alterando para número
    let mut raio: f64 = raio.trim().parse().expect("Número inválido.");

    // Cálculo da área: A = PI * r^2
    let mut area_circulo: f64 = PI * (raio*raio);
    // Impressão do resultado
    println!("Valor da área do círculo: {:.2}", area_circulo);
}
