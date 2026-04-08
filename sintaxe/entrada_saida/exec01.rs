// Eco do Usuário: Escreva um programa que peça o nome do usuário via std::io::stdin, armazene em uma String e imprima uma saudação personalizada usando a macro println!.

use std::io;

fn main(){
    println!("Digite o seu nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler.");

    println!("\nNome do usuário: {}", nome);
}