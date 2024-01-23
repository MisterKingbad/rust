mod arquivo;

use arquivo::{criar, obter_caminho_usuario, ler, existe, ler_diretorio};

fn main() {
  let caminho = obter_caminho_usuario().unwrap();

    if Ok(()) == existe(&r"../../M.txt") {
      println!("Arquivo existe!")
    } else {
      println!("Arquivo não existe!");
      criar(&caminho, &"M.txt");
    }

    ler(&r"../../M.txt");

    match ler_diretorio(&caminho) {
        Ok(_) => println!("Leitura Ok"),
        Err(_) => println!("Falha na leitura")
    }
}
