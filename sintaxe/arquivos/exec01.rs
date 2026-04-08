// Diário de Bordo: Crie um programa que:
// Crie um arquivo chamado notas.txt.
// Escreva uma frase digitada pelo usuário dentro dele.
use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main(){
    let mut arquivo = File::create("notas.txt").expect("Não foi possível criar o arquivo.");

    println!("Digite algo para salvar no arquivo .txt: ");
    let mut conteudo = String::new();
    io::stdin().read_line(&mut conteudo).expect("Falha ao ler.");

    arquivo.write_all(conteudo.as_bytes()).expect("Falha ao escrever.");
    println!("Conteúdo salvo com sucesso!!");
}