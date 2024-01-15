use std::io::stdin;

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

fn convert_to_int(data_input: &String) -> i32 {
  let x = data_input.trim().parse::<i32>().unwrap();
  x
}

fn main() {
    println!("O dobro do numero 5 é {}", dobro(5));
    println!("O maior do numero entre 5 e 4 é {}", maior(4, 5));


    let faixa = 1..=20;
    for i in faixa {
      println!("O numero esta variando entre {}", i)
    }

    let animais = vec!["Coelho", "Gato", "Macaco"];

    for a in animais {
      println!("o animal a seguir é o {}", a);
    }

    println!("============fatorial============");

      let mut entrada_fatorial = String::new();
      stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada fatorial");
      let mut fatorial = 1;
      let mut entrada_int = convert_to_int(&entrada_fatorial);

      while entrada_int > 1 {
          fatorial = fatorial * entrada_int;
          entrada_int = entrada_int - 1;
      }
      println!("O fatorial é {}", fatorial)
}
