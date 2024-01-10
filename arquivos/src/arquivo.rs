use std::{env, fs::{File, metadata, self}, io::prelude::*};

pub fn  obter_caminho_usuario() -> Option<String> {

    if let Some(caminho_home) = env::var_os("HOME") {

      Some(caminho_home.into_string().unwrap())

    } else {

      None

    }
}

pub fn criar(caminho: &str, nome_arquivo: &str) {

    println!("Criar arquivo no caminho: {}", caminho);
    println!("Criar arquivo com o nome: {}", nome_arquivo);

    let caminho_completo = format!(r"{}/{}", caminho, nome_arquivo);
    
    println!("Junção: {}",caminho_completo);

    match File::create(&caminho_completo) {
        Ok(_) => {

          println!("Arquivo criado com sucesso no caminho {}", caminho_completo)

        },
        Err(e) => {

          println!("Erro ao criar o arquivo: {}", e);

        }
    }
}

pub fn ler(caminho_completo: &str) {

  if existe(&caminho_completo).is_ok() {

    match File::open(&caminho_completo) {
      Ok(mut arquivo) => {
  
        let mut conteudo = String::new();
  
        arquivo.read_to_string(&mut conteudo).unwrap();
  
        println!("Arquivo aberto: {}", conteudo);
      },
      Err(e) => {

        println!("Erro ao ler o arquivo: {}", e);

      }
    }
  }
}

pub fn existe(caminho_completo: &str) -> Result<(), &'static str>{

  if metadata(caminho_completo).is_ok() {

      Ok(())

  } else {

      Err("O arquivo não existe.")

  }
}

pub fn ler_diretorio(caminho: &str) -> Result<(), std::io::Error> {

  let items = fs::read_dir(caminho)?;

  for i in items {
    let i = i?;

    let item_caminho = i.path();
    
    if item_caminho.is_dir() {
      println!("Diretorio: {}", item_caminho.display());
    } else {
      println!("Arquivo: {}", item_caminho.display());
    }
  }
  Ok(())
}