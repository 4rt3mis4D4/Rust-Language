//Calculadora de Soma: Peça dois números ao usuário. Como a entrada do teclado é sempre string, você precisará usar .trim().parse() para convertê-los em inteiros antes de somar.
use std:: io;

fn main(){
    println!("Digite o primeiro número: ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Falha ao ler primeiro número.");
    let num1: u64 = num1.trim().parse().expect("Falha na conversão do primeiro número.");

    println!("Digite o segundo número: ");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Falha ao ler segundo número.");
    let num2: u64 = num2.trim().parse().expect("Falha na conversão do segundo número.");
    
    let soma = num1 + num2;

    println!("\nCálculo: {} + {} = {}", num1, num2, soma);
}