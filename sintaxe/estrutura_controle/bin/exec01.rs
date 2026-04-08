// Classificador de Idade: Use if-else para dizer se uma pessoa é criança, adolescente, adulto ou idoso com base em uma variável.
use std::io;

fn main(){
    println!("Informe a sua idade: ");
    let mut idade = String::new();
    io::stdin().read_line(&mut idade).expect("Falha ao ler idade.");
    let mut idade : u64 = idade.trim().parse().expect("Falha ao converter para inteiro.");

    classificador(idade);
}

fn classificador(idade: u64){
    if idade <= 11{
        println!("Classificação: Criança.");
    } else if idade <= 17{
        println!("Classificação: Adolescente.");
    } else {
        println!("Classificação: Adulto.");
    }
}