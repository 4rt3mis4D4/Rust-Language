// Use shadowing para mudar o tipo de uma variável de String para usize.
fn main(){
    let s = String::from("Pratica com shadowing");
    println!("Valor original (string): {}", s);

    let s: usize = s.len();
    println!("Valor usize (shadowing): {}", s);
}
