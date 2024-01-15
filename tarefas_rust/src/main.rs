// Tarefa: Contando de 1 a 10 com Funções, While e For Loops em Rust

fn main() {
  usando_while();
  usando_for();

}

fn usando_while() {
  let mut i = 1;

  while i <= 10  {
    println!("usando while: {}", i);
    i += 1;
  }
}

fn usando_for() {

  for i in 1..=10  {
      println!("usando for: {}", i);
  }
}
