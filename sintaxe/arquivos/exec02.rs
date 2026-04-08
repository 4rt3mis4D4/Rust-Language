// Leitor de Configuração: Escreva um código que abra o arquivo notas.txt criado anteriormente, leia o conteúdo e o exiba no terminal.
use std::fs;

fn main(){
    let caminho = "notas.txt";
    let conteudo = fs::read_to_string(caminho).expect("Falha");
    println!("{}", conteudo);
}