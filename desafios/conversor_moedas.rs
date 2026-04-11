// 7. Conversor de Moedas com Constantes
// Defina constantes para taxas de câmbio (USD para BRL, EUR para BRL).

// Desafio: Crie uma função que aceite um valor e um "tipo de moeda" (use um char ou &str). 
// Use a estrutura match como uma expressão para atribuir o resultado da conversão diretamente a uma variável.

// Bibliotecas
use std::io;

// Constantes
const USD_BRL : f64 = 5.01;
const EUR_BRL : f64 = 5.87;

fn main(){
    // Solicitando valor para converter
    println!("Digite o valor que deseja converter: ");
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Falha ao salvar valor...");
    let valor : f64 = valor.trim().parse().expect("Falha ao converter valor para f64...");

    // Solicitando valor do tipo da moeda
    println!("Informe o tipo da moeda (USD ou EUR): ");
    let mut tipo_moeda = String::new();
    io::stdin().read_line(&mut tipo_moeda).expect("Falha ao salvar tipo da moeda...");
    let tipo_moeda = tipo_moeda.trim().to_string();

    // Chamando função
    conversor(valor, &tipo_moeda);
}

// Função de conversão
fn conversor(valor: f64, tipo: &str){
    let mut resultado: f64 = 0.0;

    match tipo.trim() {
        "USD" => {
            resultado = valor * USD_BRL;
            println!("\n=== Conversor de Moedas ===");
            println!("Valor: R$ {:.2}", valor);
            println!("Tipo de Moeda: {}", tipo);
            println!("Valor após conversão: R$ {:.2}", resultado);
        }
        "EUR" => {
            resultado = valor * EUR_BRL;
            println!("\n=== Conversor de Moedas ===");
            println!("Valor: R$ {:.2}", valor);
            println!("Tipo de Moeda: {}", tipo);
            println!("Valor após conversão: R$ {:.2}", resultado);
        }
        _ => println!("Tipo de moeda inválido."),
    }
}