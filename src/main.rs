use std::io;

fn variaveis() {
    // Maneira um
    println!("Instacia 01");

    let mut x = 4;
    println!("X is: {}", x);

    x = 5;
    println!("X is: {}", x);

    // Maneira dois
    println!("Instacia 02");

    let n = 4;
    println!("N is: {}", n);

    let n = 5;
    println!("N is: {}", n);

    // Exemplo 3
    println!("Instacia 03");

    let z = 4;
    println!("Z is: {}", z);

    let z = z + 3;
    println!("Z is: {}", z);

    // Name Shadowing exemplo
    println!("Name Shadowing exemplo");
    let a = 4;
    println!("A is: {}", a);
    {
        let a = 5;
        println!("A is: {}", a);
    }
    let a = 3;
    println!("A is: {}", a);

    {
        let a = a-1;
        println!("A is: {}", a);
    }

    let a = a+2;
    println!("A is: {}", a);
}

fn consoleInput(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("{}", input);
}

fn aritmeticas() {
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

    let food = "cookie";

    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "fruit" {
        println!("That sounds healthy!");
    } else {
        println!("oh, that's too bad!");
    }
}


fn main() {
    println!("Hello, Bem vindo ao meu projeto de estudo de Rust!");
    variaveis();
}
