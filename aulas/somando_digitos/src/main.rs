use std::io::stdin;

fn main() {
  let mut soma = 0;
  let mut valor_entrada = String::new();
  stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor_entrada");
  let mut valor_i32 = convert_to_int(&valor_entrada);
  while valor_i32 != 0 {
      let mut r = valor_i32 % 10;
      soma = soma + r;
      valor_i32 = valor_i32 / 10;

  }
  println!("O valor da soma dos digitos {}", soma);
}

fn convert_to_int(data_input: &String) -> i32 {
  let x = data_input.trim().parse::<i32>().unwrap();
  x
}