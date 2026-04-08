// Sombra (Shadowing): Crie uma variável espaco como uma string "   ". Use shadowing para transformar essa mesma variável em um inteiro que representa o tamanho da string.

fn main(){
    let s = String::from("   ");
    println!("Valor original: {}", s);

    let s: usize = s.len();
    println!("Shadowing - tamanho da string: {}", s);

}