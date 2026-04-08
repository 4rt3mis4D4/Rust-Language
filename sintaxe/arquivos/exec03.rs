// Appender: Modifique o programa para que ele não sobrescreva o arquivo, mas adicione (append) novas linhas cada vez que for executado.
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;

fn main(){
    println!("Digite algo para anexar ao arquivo: ");
    let mut conteudo = String::new();
    io::stdin().read_line(&mut conteudo).expect("Falha ao salvar.");

    let mut arquivo = OpenOptions::new().append(true).create(true).open("notas.txt").expect("Não foi possível abrir o arquivo.");

    arquivo.write_all(conteudo.as_bytes()).expect("Falha ao excrever.");

    println!("Linha adicionada com sucesso!");
}