// O Contador Imutável: Tente declarar uma variável x com valor 5 e depois alterá-la para 6. Observe o erro do compilador e corrija-o usando a palavra-chave correta.

fn main(){
    let mut x = 5;
    println!("Valor original: {}", x);

    x = 6;
    println!("Valor alterado: {}", x);
}