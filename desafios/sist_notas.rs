    // 1. O Sistema de Notas (Integração Total)
// Crie uma função calcular_media_final que receba a matricula (String), nota1, nota2 (f64) e nota_trabalho (f64).

// Desafio: A função deve retornar uma Tupla contendo a matrícula e a média final. 
// No main, utilize Shadowing para converter a média final em um conceito (A, B, C) usando match 
// Exiba o resultado formatado com 2 casas decimais.
use std::io;

fn main(){
    println!("Informe a sua matrícula: ");
    let mut matricula = String::new();
    io::stdin().read_line(&mut matricula).expect("Falha ao ler matrícula...");

    println!("Digite sua nota 1: ");
    let mut nota1 = String::new();
    io::stdin().read_line(&mut nota1).expect("Falha ao ler nota 1...");
    let nota1 : f64 = nota1.trim().parse().expect("Falha na conversão da nota 1...");

    println!("Digite sua nota 2: ");
    let mut nota2 = String::new();
    io::stdin().read_line(&mut nota2).expect("Falha ao ler nota 2...");
    let nota2 : f64 = nota2.trim().parse().expect("Falha na conversão da nota 2...");

    println!("Digite a nota do seu trabalho: ");
    let mut nota_trabalho = String::new();
    io::stdin().read_line(&mut nota_trabalho).expect("Falha ao ler nota do trabalho...");
    let nota_trabalho : f64 = nota_trabalho.trim().parse().expect("Falaha na conversão da nota do trbalho...");

    let resultado: (String, f64) = calcular_media_final(matricula, nota1, nota2, nota_trabalho);

    println!("\n=== Media Final ===");
    println!("Matrícula: {}", resultado.0);
    println!("Media Final: {:.2}", resultado.1);
}

fn calcular_media_final(m: String, n1: f64, n2: f64, nt: f64) -> (String, f64){
    let media_final : f64 = (n1 + n2 + nt) / 3.0;
    (m, media_final)
}