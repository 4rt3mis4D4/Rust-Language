    // 2. Monitor de Temperatura Mutável
// Crie um programa que armazena uma lista de temperaturas em um Array.
// Desafio: Utilize um loop para iterar sobre o array e uma Referência Mutável para converter todas as temperaturas de Celsius para Fahrenheit dentro do próprio array. 
// Use uma const para o fator de conversão ($1.8$).
const FATOR_CONVERSAO: f64 = 1.8;
const AJUSTE : f64 = 32.0;

fn main(){
    let fahrenheit = [32.0, 72.0, 98.6, 104.0, 212.0];

    println!("=== MONITOR DE TEMPERATURA MUTÁVEL ===");
    for f in fahrenheit {
        let resultado = converter_celsius(f);
        println!("FAHRENHEIT: {:.1} ---> CELSIUS: {:.1}", resultado.0, resultado.1);
    }
}

fn converter_celsius (f: f64) -> (f64, f64){
    let c = (f - AJUSTE) / FATOR_CONVERSAO;
    (f, c)
}
