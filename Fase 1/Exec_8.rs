// Desestruture uma tupla em três variáveis distintas.
fn main(){
    let tupla = (6.4, 800, "Hello");
    println!("Tupla: {:?}", tupla);

    let (a, b, c) = tupla;

    println!("Valor 1: {}", a);
    println!("Valor 2: {}", b);
    println!("Valor 3: {}", c);
}
