fn dobro(num: i32) -> i32 {
  return 2*num;
}

fn maior(a: i32, b:i32) -> i32 {
  if a >= b {
    return a;
  } else {
      return b;
  }
}

fn retorna_flot(par_a: f32, par_b: f32) -> f32 {
  println!("Essa função devolve um valor flutuante");
  let x = 10.1f32 * par_a + par_b as f32;
  x
}

fn main() {
    println!("O dobro do numero 5 é {}", dobro(5));

    println!("O maior do numero entre 5 e 4 é {}", maior(4, 5));
}
