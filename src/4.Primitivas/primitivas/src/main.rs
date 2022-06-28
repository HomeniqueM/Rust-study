fn main() {
    println!("Primitivas!");

    println!("Scalar data Type");
    println! ("Integer");
    let dec: i32 = 98_222;
    let hex: i32 = 0xff;
    let oct: i32 = 0o77;
    let bin: i32 = 0b1111_0000;
    let by:   u8 = b'A';


    println!("Decimal    : {}",dec);
    println!("Hexadecimal: {}",hex);
    println!("Octal      : {}",oct);
    println!("binario    : {}",bin);
    println!("byte       : {}",by);

    println! ("Floating");
    let floating_point32: f32 = 10.9;
    let floating_point64: f64 = 12.92223;

    println!("f32        : {}",floating_point32);
    println!("f64        : {}",floating_point64);


    println! ("Boolean");
    let true_or_false = true;
    let false_or_true: bool = false; 

    println!("Boolean    : {}",true_or_false);
    println!("Boolean    : {}",false_or_true);


    println!("Caracters");

    let letter = 'a';
    let number:char = '0';
    println!("letter    : {}",letter);
    println!("number    : {}",number);


    println!("Compound data Type");
}
