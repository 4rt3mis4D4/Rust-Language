// Slicing de String: Crie uma função que receba uma String e retorne apenas os primeiros 5 caracteres (utilize string slices &str).
fn main(){
    let mut texto: &str = "Ferrugem";
    caracteres(&mut texto);
}

fn caracteres(c: &str){
    let mut cinco_primeiros : &str = &c[0..5];
    println!("Cinco 5 caracteres: {}", cinco_primeiros);
}