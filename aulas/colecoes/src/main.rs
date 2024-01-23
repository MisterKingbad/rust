use std::collections::HashMap;

fn main() {

    println!("========================Vectors=====================");

    vetor();
    
    println!("========================Strings=====================");

    strings();
    
    println!("========================HashMap=====================");

    hashmap()
}

fn vetor() {

    let lista: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Valor na posição 2: {}", lista[2]);

    let mut numeros: Vec<u8> = Vec::new();

    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    numeros.push(4);
    numeros.push(5);
    numeros.push(6);
    numeros.push(7);
    numeros.push(8);
    numeros.push(9);
    numeros.push(10);
    
    println!("Valor do vector: {:?}", numeros);

    for n in numeros {
      println!("{}º valor", n)
    }
    
}


fn strings() {

    let mut texto = String::from("Strong");

    texto.push_str(" Gym");

    println!("{}", texto);

}

fn hashmap() {

    let mut mapa: HashMap<String, i32> = HashMap::new();

    mapa.insert("id".to_string(), 520000001);
    mapa.insert("id2".to_string(), 520000002);
    mapa.insert("id3".to_string(), 520000003);
    mapa.insert("id4".to_string(), 520000004);
    mapa.insert("id5".to_string(), 520000005);
    mapa.insert("id5".to_string(), 520000006);
    mapa.insert("id7".to_string(), 520000007);


    println!("{:?}", mapa);

}