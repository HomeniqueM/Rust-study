# Estudo de Rush

## Sumário
- [Sobre Rust](#sobre-rust)
- [Instalação em Linux](#instalação-do-rust-no-linux-arch)
- [Escrevendo código em Rust](#escrevendo-código-em-rust)
   - [Como exercultar um código é Rust](#como-exercultar-um-código-é-rust)

____
## Sobre Rust

- Fortemente Tipada 
- Linguagem compilada 
- Rapida e confiável 
- Possui características de linguagens de alto nível mas também acesso a características de baixo nível

## Instalação do Rust no linux (Arch)

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

```
Link para o site oficial com o guia de instalação caso o comando não funcione __[The Rust Programming Language](https://doc.rust-lang.org/book/ch01-01-installation.html)__

___Para Verificar se a instalação está completa___ 

Reiniciei o Terminal é digite 

```Bash
$ rustc
```
Caso aparece varias opções deu tudo certo na instalação

# Escrevendo código em Rust

A extersão usada para os arquivos rust é:  __.rs__ 

## Como exercultar um código é Rust 

```bash
rustc nomeDoArquivo.rs
```

## Controle de dependências entre outras coisas  (cargo)

Cargo vai ser usado para controle das dependências e para iniciar um projeto em Rust assim como building uma aplicação 


Para iniciar um projeto em Rust com Cargo. dentro do diretorio que você deseja criar o projeto,digite o seguinte 

```bash
cargo new nomeDoProjeto
```

Após esse comando vai ser criado uma pasta com o nome definido assim como uma subpasta ___src___ dentro dela uma __main.rs__ e junto a subpasta __src___ vai ser criando uma arquivo ___Cargo.Toml___ onde __.toml__ significa.

__Observação:__ caso o não tenha sido iniciado ele irá também iniciar um git

__t__ oms\
__o__ bious\
__m__ inimal\
__l__ anguage


Então a estrutura básica do projeto vai ser essa: 
```
project
│    Cargo.toml
└─── src     
     │ main.rs

```
Dentro da Main.rs vai

```rust
fn main() {
    println!("Hello, world!");
}

```
### Rodar um projeto com cargo
Gera o executavel 
```bash
cargo build
```

Gera o executavel e roda o projeto

```bash 
cargo run
```

Verificar se é possivel compilar o codigo( é mais rapido que bluidar o projeto)
```bash
cargo check 
```
## Rustfmt 

para formatação do codigo Rust

basta digitar no terminal junto com o arquivoa ao nome do arquivo a ser formatado e o codigo será organizado com as melhores praticas para o rust

```bash
rustfmt nomedoarquivo.rs
```

# Variáveis

Rust é uma linguagem fortemente tipada o que significa que uma vez definido um tipo a uma variável ela irá permanecer com esse tipo até o fim da vida dela.\ Outro ponto importante é que em Rust você pode definir o tipo de uma variável ou deixar que o compilador seja definido para você. 

Existe algumas forma de iniciar uma variável em Rust 

o primeiro exemplo: 

```Rust
 let x = 4;
```
Desta forma está sendo atribuído __4__ a variável __x__ e a palavras reservada ___let___ sem nenhum tipo definido, estamos indicado que para o compilador escolher o tipo da variável em tempo de compilação   

Para definimos um tipo a uma variaveis explicitamente: 

```Rust
 let x:u32 = 4;
```

dessa forma estamos definindo que x é um tipo ___integer 32__

Para pritamos uma variaveil junto a uma string 

```Rust

let x =  4;
println!("X is: {}",x);

```

### Let

Por padrão em Rust todas as variáveis são imutáveis então se você tentar fazer algo do tipo


```Rust

let x =  4;
println!("X is: {}",x);

x =  5;
println!("X is: {}",x);

```

Você vai receber um erro como esse: 


```bash 
   Compiling variaveis v0.1.0 (/home/homenique/Data/Projetos/Rust-study/src/3.Variaveis/variaveis)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x =  4;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
5 |     x = 5;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variaveis` due to previous error
```

Pararesolver esse problema é necessario adicionar o modificador __mut__ ele irá indicar que a variavel é multável durante a exercução


```Rust
let mut x =  4;
println!("X is: {}",x);

x =  5;
println!("X is: {}",x);

```

Dessa forma isso irá funcionar 

outra forma de resolver esse problema também é reinstanciar a variavel

```Rust
let n =  4;
println!("N is: {}",n);

let n = 5;
println!("N is: {}",n);

```

Dessa forma você vai sobre escrever a variavel antiga com um novo valor 

outra coisa que você pode fazer neste sentido é 

```Rust
let z = 4;
println!("Z is: {}", z);

let z = z + 3;
println!("Z is: {}", z);

```
e você vai ter como saida: 

```bash
Compiling variaveis v0.1.0 (/home/homenique/Data/Projetos/Rust-study/src/3.Variaveis/variaveis)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/variaveis`
Z is: 4
Z is: 7
```


## Name Shadowing 
Uma forma de declarar uma variável com um mesmo nome, porém com valor diferente, em um escopo menor

```Rust

let a = 4;
println!("A is: {}", a);
{
    let a = 5
    println!("A is: {}", a);
}
let a = a + 3;
println!("A is: {}", a);
```
Dessa forma temos que: 
```bash
A is: 4
A is: 5
A is: 7
```

## Constates 
Diferente de uma variáveis em rust que mesmo não podendo ser alteradas podem ser ou sobrescritas ou caso seja adicionado um modificador `mut` podendo ser alteradas, uma constante como o próprio nome diz ela é constante e imutável uma vez definida o seu valor nem seu tipo podem mudar duram toda a exercução do programa. 

Uma conversão quando é definido uma constante é usar ela toda em letras maiúsculas e usar um Underline `_` como separador 

```Rust

const SECONDS_IN_MINUTE: u32 = 60;
```
Além disso assim que é definido uma constante é necessário atribuir um tipo e também um valor a variável 


# Primitivas ( Primite data Types)

Uma primitiva é a estrutura mais simples que uma linguagem consegue representar e a partir delas podemos criar estruturas mais complexas.


Rust possui duas categorias de primitivas, __Scalar type__ e __compound__. 


## Scalar data Type
    Um tipo de dado escalar é algo que possui um conjunto finito de valores possíveis, seguindo alguma escala, ou seja, cada valor pode ser comparado a qualquer outro valor como igual, maior ou menor. Sendo eles: 
- integer (uint, int)
- floating-point
- boolean
- charecter/char

```rust
// Aqui o compilador vai usar Integer
 let x = 10;
``` 


Porém, o tipo da variável pode ser definido explicitamente 

```rust
// Aqui eu estou explicitamente definido que o x é um Integer de 32 bits
 let x:i32 = 2;
``` 
Nessa definição temos dois pontos o `i` que indica que essa variável é um Inteiro com sinal e `32` para indicar o número de bits que ela vai utilizado para armazenar e representar aquele valor. 


__Obs__: 32bits é o valor padrão para um inteiro em Rust

Em Rust também é possível utilizar valores inteiro não negativos chamados de unsigned que a declaração é similar ao integer só que em vez de usar `i` vai ser utilizado `u` 

### Integer Types

Essa são as formas de declaração de inteiros

| Tamanho | Com Sinal | Sem Sinal |
|---------|-----------|-----------|
| 8-bit   | i8        | u8        |
| 16-bit  | i16       | u16       |
| 32-bit  | i32       | u32       |
| 64-bit  | i64       | u64       |
| 128-bit | i128      | u128      |
| arch    | isize     | usize     |

Os tipos `isize` e `usize` dependem da arquitetura do
computador em que seu programa está sendo executado, que é indicado na tabela como “arch”:
64 bits se você estiver em uma arquitetura de 64 bits e 32 bits se estiver em uma arquitetura de 32 bits
arquitetura.

Você pode escrever literais inteiros em qualquer uma das formas mostradas a seguir. 

Observação: que os literais numéricos que podem ser vários tipos numéricos permitem um sufixo de tipo,
como `57u8`, para designar o tipo. Os literais numéricos também podem usar `_` como
separador visual para facilitar a leitura do número, como `1_000`, que
têm o mesmo valor como se você tivesse especificado `1000`.


| Number literals    | Example       |
|--------------------|---------------|
| Decimal            | `98_222`      |
| Hex                | `0xff`        |
| Octal              | `0o77`        |
| Binary             | `0b1111_0000` |
| Byte (apenas `u8`) | `b'A'`        |


### Floating-Point Types
Rust também tem dois tipos primitivos para números de ponto flutuante , que são números com pontos decimais. Os tipos de ponto flutuante do Rust são `f32` e `f64`, que têm 32 bits e 64 bits de tamanho, respectivamente.

| Tamanho | Represetação |
|---------|--------------|
| 32-bit  | f32          |
| 64-bit  | f64          |



### Booleano

Como na maioria das outras linguagens de programação, um tipo booleano em Rust tem dois valores possíveis: `true` e `false`. Booleanos têm um byte de tamanho. O tipo booleano em Rust é especificado usando `bool`. Por exemplo:

```Rust

fn main() {
    let true_or_false = true;

    let true_or_false: bool = false; // with explicit type annotation
}

```

### Characters

O tipo de Rust `char` é o tipo alfabético mais primitivo da linguagem. Veja alguns exemplos de declaração de charvalores:

```Rust
fn main() {
    let letter = 'a';
    let number:char = '0';
}
```
Observe que os chars literais são declarados com aspas simples, em oposição a literais de string, que usam aspas duplas. 

O tipo `char` tem quatro bytes de tamanho e representa um valor escalar Unicode, o que significa que pode representar muito mais do que apenas ASCII. 


## Compound data Type
    Um tipo de dados composto é qualquer tipo de dados que pode ser construído em um programa usando os tipos de dados primitivos da linguagem de programação e outros tipos compostos. e em Rust temos :

- tuple
- array
Como já dito, em Rust você pode definir implicitamente o tipo deixando para o compilador decidir qual o tipo da variável.


### Tuple 

De Forma Simples podemos dizer que uma tupula é uma estrutura que suporta varios tipos de valores 

Por exemplo podemos criar uma tupula desta forma

```Rust
fn main() {
   // Desta forma estamos explicitado os tipos
    let tup:(i32,bool,char) = (1,true,'S');
}

```
A variável `tup` liga-se a tupla, porque uma tupla é considerada um único elemento composto. Para pegar os valores da tupla individualmente, podemos usar a correspondência de padrões para desestruturar o valor de uma tupla, como este:

```Rust
fn main( ){
     let tup:(i32,bool,char) = (1,true,'S');
     let i,b,c = tup;
     println!("O valor do c é: {}", c);

}
```

Além de desestruturar através da correspondência de padrões, podemos acessar diretamente um elemento da tupla usando um ponto (`.`) como índice do valor que queremos acessar. Por exemplo:

```rust 
    let tup:(i32,bool,char) = (1,true,'S');
    println!("{}",tup.0);
    println!("{}",tup.1);
```

Alguns pontos importante que devemos falar antes de seguir em frente:

A vida de uma tupula é igual a uma variavel, caso não seja adicionado o modificador de mutabilidade `mut`
```Rust
    let tup:(i32,bool,char) = (1,true,'S');
    tup.0 = 10;
```
o erro que vamos obter:

```Bash 
error[E0594]: cannot assign to `tup.0`, as `tup` is not declared as mutable
  --> src/main.rs:59:5
   |
58 |     let tup:(i32,bool,char) = (1,true,'S');
   |         --- help: consider changing this to be mutable: `mut tup`
59 |     tup.0 = 10;
   |     ^^^^^^^^^^ cannot assign
```
então para resolver esse problema basta adicioar o modificador de mutabilidade.

```rust
    let mut tup:(i32,bool,char) = (1,true,'S');
    tup.0 = 10;

    println!("{}",tup.0);
```

Outra situação é são tupalas parecidas mas são diferentes


```Rust
fn main (){

    let tup:(i32,bool,char) = (1,true,'S');
    
    // Tuples diferentes, não podem se relacionar
    let tup2:(i8,bool,char) = (1,true,'S');
}
```

## Matriz
Diferente de uma tupula um array suporta somente um tipo dentro de sua estrutura  

```Rust
fn main(){
	// Para iniciar um array
	let arr = [ 1, 2, 3, 4, 5];
	
// Acessar o primeiro valor do Array

arr[0];

// Acessar o terceiro
arr[2];
}
```

  Como podemos ver no exemplo, podemos acessar os valores do array do array através do uso de colchetes ‘[ ]’.

 Diferente de outras linguagens programação não é possível adicionar novos valores ao array, é necessário reescrever o array com um tamanho de alocação maior, além das mesma regras de mutação vales para o array 


Como caveiras e tupulas é possível definir o array de forma explícita de algumas maneiras diferentes

```Rust
fn main(){
	// Para iniciar um array
	le mut arr [i32;5] = [ 1, 2, 3, 4, 5];
	
}
```
Desta forma temos definido um array de `i32` com tamanho 5 e já definido os valores para cada posição 

podemos também iniciar desta forma

```Rust
fn main(){
	// Para iniciar um array
	le mut arr [i32;5] ;
}
```

Podemos iniciar desta forma onde igual ao exemplo anterior vamos ter um array de de `i32` com tamanho 5 porém com nenhum valor iniciado, desta forma caso tentamos mostrar algum valor teremos um erro do compilador.



outro cenário que irá mostrar erro é caso seja atribuindo um colchete vazio para um array 
```Rust
fn main(){
	// Irá apresentar error 
	le mut arr [i32;5]  = [ ];
}
```

  Para mostramos todos os valores de um array diferente de outras linguagem não podemos simplesmente colocar dentro da função de print

```Rust
fn main(){
	// Irá apresentar error 
	le mut arr [i32;5]  = [ ];
   	println!("{}",arr);
}
```

isso vai gerar outro erro de compilação, é necessário indicar qual posição vai ser printanda do array

```Rust
fn main(){
	// Irá apresentar error 
	le mut arr [i32;5]  = [ ];
   	println!("{}",arr[0]);
}
```
___
## Console Input 
Na linguagem Rust existem alguns pacotes e funções que já vem como padrão conhecidos como __prelude__. Porém o user input não está incluindo nesse __prelude__, e é necessários importar esse pacote que em rust, todo pacote e ou biblioteca em rust é denominado  `crate`. Dentro desse `crete` temos módulos que são funcionalidades específicas   


Por exemplo vamos usar `IO modulo`  para input do usuário:

``` Rust
use std::io; 

```

O que está acontecendo na primeira linha é você está importando o módulo `IO` da biblioteca (crate) std ( standard library) 

### Pegando  Input do usuário


``` Rust
use std::io; 

fn main (){
     let mut input = String::new();
}
```  
Para fazer uma leitura de usuário uma variável necessariamente tem que ser mutável, além disso neste exemplo vemos que estamos chamando o módulo new para instanciar uma nova string 


Para coletar a entrada 

``` Rust
use std::io; 

fn main (){
     let mut input = String::new();

    io::stdin().read_line(&mut  input).expect("failed to read line");

    println!("{}", input);
}
```  

Ok, está acontecendo muitas coisas nessa operação, vamos tentar entender por partes 

- ` io::stdin().read_line(` : Basicamente, estamos aqui a partir do módulo de `IO` estamos chamando a função de readline.
- `&mut  input` : quando estamos passando por parâmetro uma variável, estamos na verdade criando uma cópia da mesma para ser alterada dentro da função que não afeta a variável original, por isso usamos o operador `&` para enviar uma referência da variável, porém em rust referências não podem ser alterada a não ser que tenho o modificador `mut` .

Desta forma estamos enviando uma referência de onde está a variável e tornamos essa referência mutável. 

- `.expect(“failed to read line”);` : caso aconteça algum erro durante a leitura do input vai ser enviado a mensagem definida por nós, no caso  ‘failed to read line’

## Operações Aritméticas e Type Casting  
 Um ponto muito importante em operações aritméticas em Rust é que o resultado de cada operação vai ser igual ao tipo do valores que são fatores da operação 


por exemplo se fizermos a divisão de dois valores inteiro o resultado é um valor inteiro 
```rust
fn main() {
    let x: u8 = 255; // 0 - 255
    let y: u8 = 10; // 0 - 255

    let z = x / y;
    println!("{}",z);
}
```

No terminal vamos ter que o resultado desta operação é `25`, ou seja foi descartado o decimal, pois o tipo `u8` não suporta valores com ponto flutuante.  

Por padrão Rust suporta 5 operações básicas aritméticas  

``` Rust
fn main() {
    // Adição
    let sum = 5 + 10;

    // Subtração
    let difference = 95.5 - 4.3;

    // Multiplicação
    let product = 4 * 30;

    // Divisão
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // módulo (Resto da divisão)
    let remainder = 43 % 5;
}
```
### Possíveis erros em operações aritméticas 
Caso tentamos fazer operações com tipos inteiros com aplituidade diferentes como um `u8` somando com um `i8`

```rust
fn main() {
    let x: u8 = 9; // 0 - 255
    let y: i8 = 10; // -128 -127

    let z = x + y;
    println!("{}",z);
}
```
Vamos obter esse erro: 
```bash 
error[E0308]: mismatched types
 --> src/main.rs:5:17
  |
5 |     let z = x + y;
  |                 ^ expected `u8`, found `i8`

error[E0277]: cannot add `i8` to `u8`
 --> src/main.rs:5:15
  |
5 |     let z = x + y;
  |               ^ no implementation for `u8 + i8`
  |
  = help: the trait `Add<i8>` is not implemented for `u8`

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `arithmetic` due to 2 previous errors
```
a mesma coisa vai acontecer quando o número de bits for diferente, o mesmo erro de não ter um operação implementada 

```rust
fn main() {
    let x: i8 = 9;
    let y: i64 = 10;

    let z = x + y;
    println!("{}",z);
}
```
Esse problema também ocorre com os floats 

 ```rust
fn main() {
    let x: f32 = 9.0;
    let y: f64 = 10.0;

    let z = x + y;
    println!("{}",z);
}
```
Para essas situações, uma forma de resolver esse problema é converter esse valores para o mesmo tipo.


Outro erro que podem acontecer é você tentar somar dois literais que vão estourar o limite de uma variável 
```rust
fn main() {
    let x: u8 = 255; // 0 - 255
    let y: u8 = 1; // 0 - 255

    let z = x + y;
    println!("{}",z);
}
```

O compilador irá te devolver esse erro, indicando que você estourou o limite da variável 

```bash
error: this arithmetic operation will overflow
 --> src/main.rs:5:13
  |
5 |     let z = x + y;
  |             ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default

error: could not compile `arithmetic` due to previous error
```

Novamente a solução vai ser fazer um cast para um tipo maior que vá caber este valor


## Type Casting

Type casting é o ato de converter um tipo em outro
uma maneira bem simples de fazer isso é apos o valor informar o tipo 

```rust
fn main() {
    let x = 255.0 f32;
    let y = 1.0 f32; 
    let z = x + y;
    println!("{}",z);
}
```
uma segunda maneira de fazer o type casting após o valor você ultilizar `_` e após isso o tipo:

```rust
fn main() {
    let x = 127_i8;
    let y = 10_i8; 
    let z = x + y;
    println!("{}",z);
}
```

A terceira maneira de fazer um type casting é em vez de usar o underline `_` é utilizar a palavra reservada `as` 
```rust
fn main() {
    let x = 127_000 as i64;
    let y = 10 as i64; 
    let z = x + y;
    println!("{}",z);
}
```

outro exemplos 

```rust
fn main(){
    let x = 127_000 as i64;
    let y = 12_i32;
    println!("x: {} ",x);
    println!("y: {} ",y);

    let z = x/(y as i64);
    println!("Resultado: {} ",z);
}
```
Em geral sempre que for fazer uma conversão de um tipo para outro, sempre dê preferência para alterar o menor tipo para o maior ,pois isso evita possíveis perda de informação ou pior, um buffer overflow. Onde você estoura o limite da variável.

```Rust
// buffer overflow
let x = (i32::MAX as i64) +1;
let y = 10_i32;

let z = (x as i32)/y;

println!(z);
```


### converter uma string em uma

```rust
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("expected to read line");
    let int_input:i64 = input.trim().parse().unwrap();

    println!("lido: {}", input)


```

- `trim()`
- `parse()`
- `unwrap()`