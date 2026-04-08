// O Poder do Match: Crie uma variável cor (string). Use a estrutura match para imprimir o código Hexadecimal da cor. Não esqueça de tratar o caso padrão (o "else" do match) com _.
use std::io;

fn main(){
    println!("Digite uma cor primaria (vermelho, verde, azul): ");
    let mut cor = String::new();
    io::stdin().read_line(&mut cor).expect("Falha ao ler cor.");
    
    match cor.trim().to_uppercase().as_str() {
        "VERMELHO" => println!("\n{} - #FF0000", cor.trim().to_uppercase().as_str()),
        "VERDE" => println!("\n{} - #00FF00", cor.trim().to_uppercase().as_str()),
        "AZUL" => println!("\n{} - #0000FF", cor.trim().to_uppercase().as_str()),
        _ => println!("Não é uma cor primária."),
    }
}

