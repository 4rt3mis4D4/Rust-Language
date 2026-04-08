// Arrays vs Matrizes: Declare um array de inteiros com 5 elementos fixos. Tente acessar um índice inexistente e observe o que acontece (panic).
fn main(){
    let var = [1, 2, 3, 4, 5];
    println!("Acesso ao indíce: {}", var[3]);
}