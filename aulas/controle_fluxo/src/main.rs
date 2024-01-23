fn main() {
    se();
    declaracao_if();
    loop_infinito();
    while_finito();
    for_interator();
    deu_match();
}

fn se() {

  let numero = 0;

  if numero  > 0 {
    println!("O numero {} é positivo.", numero)
  } else if numero == 0 {
    println!("O numero é zero.")
  }
  else {  
    println!("O numero {} é negativo.", numero)
  }

}

fn declaracao_if() {

  let condicao = true;
  let  resultado = if condicao {
    "A condição e verdadeira"
  } else {
    "A condição e falsa"  
  };

  println!("{}", resultado)
}

fn loop_infinito() {

  let mut contador = 0;
  loop {
    println!("Contador: {}", contador);

    contador += 1;

    if contador == 5 {
      break;
    }
  }
}

fn while_finito() {
  let mut contador = 0;

  while contador < 5 {
    println!("Contador: {}", contador);

    contador += 1;
  }
}

fn for_interator() {

  for i in 0..=10 {
    println!("o index é {}", i)
  }
}

fn deu_match() {
  let estacao_atual = "verão";

  match estacao_atual {
    "primavera" => {
      println!("É primavera");
    },
    "verão" => {
      println!("É verão");
    },
    "outono" => {
      println!("É outono");
    },
    "inverno" => {
      println!("É inverno");
    },
    _ => {
      println!("Não sei")
    }
  }
}


