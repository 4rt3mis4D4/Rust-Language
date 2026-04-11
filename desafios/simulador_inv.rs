// 3. Simulador de Inventário com Ownership
// Desenvolva uma função que receba a posse (Ownership) de uma String representando um item de inventário e um i32 para quantidade.

// Desafio: A função deve verificar se a quantidade é maior que zero. Se for, retorna uma nova String formatada; 
// Caso contrário, retorna uma mensagem de erro. 
// Tente usar o item no main após chamar a função para observar o erro de movimentação de posse.
use std::io;

fn main(){
    println!("Informe o nome do item: ");
    let mut item = String::new();
    io::stdin().read_line(&mut item).expect("Falha ao ler item...");
    
    println!("Informe a quantidade: ");
    let mut quantidade = String::new();
    io::stdin().read_line(&mut quantidade).expect("Falha ao ler quantidade...");
    let quantidade = quantidade.trim().parse().expect("Falha ao converter quantidade...");

    let resultado = simulador(item.trim().to_string(), quantidade);

    if resultado.1 == true{
        println!("Sucesso!! {}", resultado.0);
    } else {
        println!("{}", resultado.0);
    }
}

fn simulador(item: String, quant: i32) -> (String, bool){
    if quant > 0{
       (format!("Item: {} | Quantidade: {}", item, quant), true) 
    } else {
        (String::from("Quantidade inválida."), false)
    }
}