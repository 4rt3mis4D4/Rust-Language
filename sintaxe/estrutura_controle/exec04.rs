// Loop com Retorno: Use a estrutura loop para incrementar um contador. Quando ele chegar a 10, use break para retornar o valor dobrado e armazene-o em uma variável.
fn main(){
    let mut contador: u64 = 0;

    let num = loop{
        contador +=1;

        if contador == 10{
            break contador * 2;
        }
    };

    println!("Número dobrado: {}", num);
}