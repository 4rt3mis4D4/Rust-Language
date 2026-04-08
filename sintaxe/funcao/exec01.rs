// Fahrenheit para Celsius: Escreva uma função converter_clima(f: f64) -> f64. Lembre-se que em Rust, a última expressão sem ponto e vírgula é o retorno da função.
fn main(){
    let temp_f: f64 = 86.0;
    let resultado: f64 = converter_clima(temp_f);

    println!("Fahrenheit: {} para Celsius: {}", temp_f, resultado);
}

fn converter_clima(f: f64) -> f64{
    let c : f64 = (f-32.0)*0.55;
    c
}

