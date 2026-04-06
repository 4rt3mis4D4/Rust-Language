// Calcule o resto da divisão de dois números inteiros.
use std::io;

fn main(){
    println!("Informe o primeiro número: ");
    let mut num_um = String::new();
    io::stdin().read_line(&mut num_um).expect("Falha na leitura.");

    let mut num_um: u64 = num_um.trim().parse().expect("Falha na conversão.");

    println!("Informe o segundo número: ");
    let mut num_dois = String::new();
    io::stdin().read_line(&mut num_dois).expect("Falha na leitura.");

    let mut num_dois: u64 = num_dois.trim().parse().expect("Falha na conversão");

    let mut resto = num_um % num_dois;

    println!("{} % {}: {}", num_um, num_dois, resto);
}
