// A Tupla de Dados: Crie uma tupla que armazene o nome de um produto (string), seu preço (f64) e se está em estoque (bool). Extraia os valores usando padrão de desestruturação e por índice.
fn main(){
    let tup = ("teclado", 50.2, true);
    println!("Valores da Tupla: {:?}", tup);

    let (produto, preco, estoque) = tup;
    println!("\nProduto: {}", produto);
    println!("Preço: {}", preco);
    println!("Em estoque: {}", estoque);

    println!("\nAcesso com o indíce:");
    println!("Produto: {} - Preco: R${} - Em estoque: {}", tup.0, tup.1, tup.2)
}