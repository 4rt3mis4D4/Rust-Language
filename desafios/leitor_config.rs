// 4. Leitor de Configurações via Terminal
// Crie um pequeno script que solicita ao usuário seu nome, idade e linguagem de programação favorita via stdin.

// Desafio: Salve esses dados em uma Struct ou Tupla. 
// Use match para validar se a idade inserida é um número válido e trate o erro caso o usuário digite letras.
use std::io;

fn main(){
    println!("Digite o seu nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler nome...");
    let nome = nome.trim().to_string();

    let idade_final : u8;

    loop {
        println!("Digite sua idade: ");
        let mut idade = String::new();
        io::stdin().read_line(&mut idade).expect("Falha ao ler idade...");
        
        match idade.trim().parse::<u8>(){
            Ok(num) => {
                idade_final = num;
                break;
            }
            Err(_) => {
                println!("Idade inválida. Tente novamente.");
            }
        }
    }

    println!("Digite a sua linguagem de programação favorita: ");
    let mut linguagem = String::new();
    io::stdin().read_line(&mut linguagem).expect("Falha ao ler linguagem de programação...");
    let linguagem = linguagem.trim().to_string();

    let dados = (nome, idade_final, linguagem);

    println!("\n=== CONFIGURAÇÃO DO USUÁRIO ===");
    println!("Nome: {}", dados.0);
    println!("Idade: {}", dados.1);
    println!("Linguagem de Programação Favorita: {}", dados.2);
}