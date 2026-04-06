// Use a função std::mem::size_of_val para verificar o tamanho em bytes de um i64 vs um i8.
use std::mem;

fn main(){
    println!("Tamanho em bytes i64: {}", mem::size_of_val(&1i64));
    println!("Tamanho em bytes i8: {}", mem::size_of_val(&1i8));
}
