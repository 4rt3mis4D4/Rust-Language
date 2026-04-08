// Verificador de Paridade: Crie uma função que recebe um inteiro e retorna um booleano. Use-a dentro de um if.
fn main(){
    let num : i64 = 5;
    
    if par(num) == true{
        println!("O número {} é par.", num);
    } else {
        println!("O número {} é ímpar.", num);
    }
}

fn par(numero: i64) -> bool{
    numero % 2 == 0
}