// Desafio Final: O Sistema de Inventário Simples
// Crie um programa que consolide tudo:

// O usuário escolhe entre: (1) Adicionar Item, (2) Listar Itens, (3) Salvar em Arquivo, (4) Sair.
// Use um match para lidar com a escolha.
// Os itens podem ser armazenados em um Array ou apenas impressos.
// Ao salvar, escreva os dados em um arquivo .txt.
use std::io;
use std::process;
use std::fs::File;
use std::io::Write;

fn main(){
    let mut itens : [String; 5] = Default::default(); // Array de itens
    let mut contador = 0;
    let mut arquivo = File::create("itens.txt").expect("Não foi possível criar o arquivo.");

    loop{
        // Menu
        println!("=== Sistema de Inventário ===");
        println!("(1) Adicionar Item");
        println!("(2) Listar Itens");
        println!("(3) Salvar em Arquivo");
        println!("(4) Sair");
        println!("Digite a opção: ");
        // Opção escolhida
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha na escolha de opção...");

        // Escolhas
        match opcao.trim() {
            // ---Adicionar
            "1" => { println!("\n=== (1) Adicionar Item ===\n");
                    if contador < itens.len(){
                        println!("Digite o nome do item para adicionar: ");
                        let mut novo_item = String::new();
                        io::stdin().read_line(&mut novo_item).expect("Falha ao adicionar novo item...");
                        
                        itens[contador] = novo_item.trim().to_string();

                        println!("\nItem: {} adicionado com sucesso! Posição: {}\n\n", itens[contador], contador);
                        contador += 1;
                    } else {
                        println!("Limite de itens atingido!\n");
                    }
            },
            // ---Listar
            "2" => { println!("\n=== (2) Listar Itens ===\n");
                    println!("Itens: {:?}", itens);
            },
            // ---Salvar
            "3" => { println!("\n=== (3) Salvar em Arquivo ===\n");
                    let conteudo = itens.join("\n");

                    arquivo.write_all(conteudo.as_bytes()).expect("Falha ao salvar itens no arquivo.");
                    println!("Itens salvos com sucesso!!\n");
            }, 
            // ---Sair
            "4" => { println!("\n=== (4) Sair ===\n");
                    println!("Encerrando programa...\n");
                    break;
            },
            // ---Nenhuma das opções
            _ => println!("Opção Inválida!!!\n"),
        }
    }
}
