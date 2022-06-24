fn main() {
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
