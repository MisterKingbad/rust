enum Fruta {
    Maca,
    Banana,
    Laranja,
    Morango,
    Acai 
}

enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32)
}

struct Pessoa {
    nome: String,
    idade: i8,
    altura: f32
}

struct Retangulo {
    altura: u32,
    largura: u32 
}

trait FormaGeometrica {
    
    fn calcular_area(&self) -> u32;

    fn new(largura: u32, altura: u32) -> Self;

}

impl FormaGeometrica for Retangulo {
    
    fn calcular_area(&self) -> u32 {
      self.largura * self.altura
    }

    fn new(largura: u32, altura: u32) -> Self {
      Self{altura, largura}
    }
}

fn main() {
    enumeracao(Fruta::Maca);
    enumeracao(Fruta::Banana);
    enumeracao(Fruta::Laranja);
    enumeracao(Fruta::Morango);
    enumeracao(Fruta::Acai);

    enumeracao_com_dados();
    estrutura();
}

fn enumeracao(fruta: Fruta) {
    match fruta {
        Fruta::Maca => println!("É uma maca"),
        Fruta::Banana => println!("É uma banana"),
        Fruta::Laranja => println!("É uma laranja"),
        Fruta::Morango => println!("É um morango"),
        Fruta::Acai => println!("É um acai"),
    }

}

fn enumeracao_com_dados() {
    let ponto2d = Coordenada::DoisD(5, 10);
    let ponto3d = Coordenada::TresD(3, 8, 15);

    match ponto2d {
        Coordenada::DoisD(x, y) => println!("Coordenada 2d: {}, {}", x, y),
        Coordenada::TresD(_, _, _) => println!("Coordenada 3d")
    }

    match ponto3d {
        Coordenada::DoisD(_, _) => println!("Coordenada 2d"),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3d: {}, {}, {}", x, y, z)
    }
}

fn estrutura() {
  let gaucho = Pessoa{
    nome: String::from("Gaucho"), 
    idade: 40,
    altura: 1.71
  };

  println!("Nome: {}", gaucho.nome);
  println!("Idade: {}", gaucho.idade);
  println!("Altura: {}", gaucho.altura);

  let retangulo1 = Retangulo{
    largura: 10,
    altura: 20
  };

  let area1 = retangulo1.calcular_area();

  println!("React 1: {}", area1);

  let retangulo2 = Retangulo{
    largura: 33,
    altura: 5
  };
  
  let area2 = retangulo2.calcular_area();

  println!("React 2: {}", area2);

  let retangulo3 = Retangulo::new(10, 20);
  
  let area3 = retangulo3.calcular_area();

  println!("React 3: {}", area3);

}
