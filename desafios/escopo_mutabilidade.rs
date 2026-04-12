// 10. Desafio de Escopo e Mutabilidade
// Crie uma variável mutável pontuacao. Dentro de um novo bloco de escopo { }, 
// utilize Shadowing para criar uma nova versão dessa variável (com tipo diferente, como String).

// Desafio: Após o fechamento do bloco, tente realizar uma operação aritmética com a pontuacao original no escopo externo. 
// Explique via comentário no código por que a variável original ainda mantém seu valor e tipo inicial.

// Biblioteca
use std::io;

fn main(){
    // Criação da variável pontuação
    let mut pontuacao: u64 = 97;
    println!("Valor da variável original: {}", pontuacao);

    // Escopo
    {
        let pontuacao = String::from("Noventa e Sete.");
        println!("Valor da variável Shadowing: {}", pontuacao);
    }

    // Operação Artmética
    let operacao_aritmetica = pontuacao + 3;
    println!("Resultado: {}", operacao_aritmetica);
}

/*
    Explicação:
    Shadowing não altera o valor original da variável, apenas esconde/sombreia o valor anterior enquanto o bloco estiver ativo,
    ou seja, e após o fechamento do escopo a variável Shadowing é descartada da memória e mantém o seu valor original.
*/
