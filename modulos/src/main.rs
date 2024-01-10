mod operacoes;
use operacoes::matematica::{somar, subtrair};
use rand::random;


fn main() {
    println!("2 + 2 = {}", somar(2, 2));
    println!("2 - 2 = {}", subtrair(2, 2));

    println!("Numero aleatorio: {}", random::<f32>())
}
