//Crie uma tupla com três tipos diferentes e acesse seus valores individualmente.
fn main(){
    let tupla = (500, 6.4, "Hello");
    println!("Tupla: {:?}", tupla);

    println!("Primeiro valor: {}", tupla.0);
    println!("Segundo valor: {}", tupla.1);
    println!("Terceiro valor: {}", tupla.2);
}
