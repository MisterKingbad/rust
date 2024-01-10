fn main() {
  nome_da_funcao();

  let a = com_return();
  let b = sem_return();

  println!("O valor retornado em a é: {}", a);
  println!("O valor retornado em b é: {}", b);

  let c = maior_valor(2000, 2001);

  println!("O maior valor de c é: {}", c);

  let d = 5;

  let resultado_incremento = incrementa(d);

  println!("O numero original: {}, Numero incrementado {}", d, resultado_incremento);
  
}

fn nome_da_funcao() {
  println!("Ola King!");
}

fn com_return() -> i8 {
  return 3;
}

fn sem_return() -> i8 {
  3
}

fn maior_valor(a: i32, b:i32) -> i32 {
  {
    if a > b {
        a
    } else {
        b
    }
  }
}

fn incrementa(mut a: i32) -> i32 {
  a += 1;
  a
}
