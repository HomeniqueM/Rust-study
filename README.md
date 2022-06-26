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

Scalar data Type:

    Um tipo de dado escalar é algo que possui um conjunto finito de valores possíveis, seguindo alguma escala, ou seja, cada valor pode ser comparado a qualquer outro valor como igual, maior ou menor. Sendo eles: 
- integer (uint, int)
- floating-point
- boolean
- charecter/char

Compound data Type:

    Um tipo de dados composto é qualquer tipo de dados que pode ser construído em um programa usando os tipos de dados primitivos da linguagem de programação e outros tipos compostos. e em Rust temos :

- tuple
- array
Como já dito, em Rust você pode definir implicitamente o tipo deixando para o compilador decidir qual o tipo da variável.

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