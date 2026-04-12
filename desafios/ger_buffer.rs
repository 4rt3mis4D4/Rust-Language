// O Gerenciador de Buffer (Loops e I/O)
// Crie um programa que aceite entradas do usuário continuamente dentro de um while.

// Desafio: Cada entrada deve ser anexada a uma String principal (buffer). 
// O loop só deve parar quando o usuário digitar "SAIR". 
// Ao final, salve todo o conteúdo acumulado em um arquivo sessao.txt.

// Bibliotecas
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut entradas = String::new(); // Entradas do usuário

    let mut arquivo = File::create("sessao.txt").expect("Falha ao criar arquivo: sessao.txt..."); // Criação do arquivo

    // Loop que se encerra quando o usuário digita "SAIR"
    while entradas.trim() != "SAIR"{
        println!("\nDigite a entrada ou digite 'SAIR' para encerrar: "); // Solicitando entradas
        entradas.clear(); // Retirada de lixo
        io::stdin().read_line(&mut entradas).expect("Falha ao salvar entrada..."); // Salvando entradas

        if entradas.trim() != "SAIR" {
            arquivo.write_all(entradas.as_bytes()).expect("Falha ao escrever no arquivo..."); // Escrevendo no arquivo
        }
    }

    // Fim do programa
    println!("\nFim do programa...");
}