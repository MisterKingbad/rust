use std::io::stdin;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn main() {
    let mut numero = String::new();
    stdin().read_line(&mut numero).expect("Erro ao ler numero");
    let mut numero2 = String::new();
    stdin().read_line(&mut numero2).expect("Erro ao ler numero");
    if convert_to_int(&numero) > convert_to_int(&numero2) {
        println!("O numero {} é maior que {}", numero, numero2);
    } else {
        println!("O numero {} é menor ou igual que {}", numero, numero2);
    }
  }