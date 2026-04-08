// FizzBuzz Rustaceano: Use um loop for de 1 a 50. Se o número for divisível por 3, imprima "Fizz"; por 5, "Buzz"; por ambos, "FizzBuzz".
fn main(){
   for numero in 1 ..= 50{
        if numero % 3 == 0 && numero % 5 == 0{
            println!("FizzBuzz");
        } else if numero % 3 == 0{
            println!("Fizz");
        } else if numero % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}", numero);
        }
   }
}