use std::io;

// fn convert_to_int(data_input: &String) -> i32 {
//   let x = data_input.trim().parse::<i32>().unwrap();
//   x
// }

fn main() {
    let mut a = 15;
    let mut b = 40;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    println!("O maior divisor comum entre 15 e 40 é: {}", a)

}
