//Converta explicitamente um f64 para um u8 usando a palavra-chave as.
fn main(){
    let var_f64: f64 = 8.6;
    println!("Valor original: {}", var_f64);
    
    let var_u8 = var_f64 as u8;
    println!("Valor alterado: {}", var_u8);
}
