// 6. Processador de Arquivo de Log
// Crie um programa que lê um arquivo chamado log.txt.

// Desafio: Use um laço for para ler linha por linha. Se a linha contiver a palavra "ERRO", 
// salve essa linha em um novo arquivo chamado erros_criticos.txt. 
// Utilize as funções de std::fs e trate os retornos com if let ou match.

// Bibliotecas
use std::fs::File; 
use std::io::prelude::*;
use std::io::{self, BufRead};

fn main(){
    // Declarando caminho
    let caminho = "log.txt";

    // Criando arquivo erros_criticos.txt
    let mut arquivo_erro = File::create("erros_criticos.txt").expect("Falha ao criar arquivo: erros_criticos.txt");

    // Abrir arquivo log.txt
    let arquivo = File::open(caminho).expect("Falha ao abrir arquivo log.txt");
    
    // Leitor para cada linha
    let leitor = io::BufReader::new(arquivo);

    // Leitura linha por linha
    for linha in leitor.lines(){
        if let Ok(conteudo) = linha{
            if conteudo.contains("ERRO"){
                println!("Erro encontrado: {}", conteudo);

                let quebra_linha = format!("{}\n", conteudo);

                arquivo_erro.write_all(quebra_linha.as_bytes()).expect("Falha ao salvar no arquivo: erros_criticos.txt");
            }   
        }
    }

    // Fim
    println!("Fim do programa...");

}
