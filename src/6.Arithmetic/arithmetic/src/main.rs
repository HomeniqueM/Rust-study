use std::io;

fn main() {
    println!("Operações: ");
    let x = 255.0; // 0 - 255
    let y = 10.0; // 0 - 255

    // Adição
    let sum = 5 + 10;
    println!("{} + {} = {}", x, y, sum);

    // Subtração
    let difference = 95.5 - 4.3;
    println!("{} - {} = {}", x, y, difference);

    // Multiplicação
    let product = 4 * 30;
    println!("{} * {} = {}", x, y, product);

    // Divisão
    let quotient = x / y;
    println!("{} / {} = {}", x, y, quotient);

    // módulo (Resto da divisão)
    let remainder = x % y;
    println!("{} % {} = {}", x, y, remainder);

    println!("Type Casting: ");

    let x = 127_000 as i64;
    let y = 12_i32;
    println!("x: {} ", x);
    println!("y: {} ", y);

    let z = x / (y as i64);
    println!("Resultado: {} ", z);

    // BuffOverflow
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;

    let z = (x as i32) / y;

    println!("{}", z);

    println!("Type Casting String para um valor numerico ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");
    let int_input: i64 = input.trim().parse().unwrap();

    println!("lido: {}", input);

    println!("Condicionais");

    let fond = "cookie";

    if fond == "cookie" {
        println!("I like cookies too!");
    } else if food == "fruit" {
        println!("That sounds healthy!");
    } else {
        println!("oh, that's too bad!");
    }
}
