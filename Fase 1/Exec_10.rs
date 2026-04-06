// Tente acessar um índice fora dos limites de um array e observe o que acontece no runtime.
fn main(){
    let array = [8, 2, 70, 717, 90];
    println!("{}", array[9]); // Error: index out of bounds
}
