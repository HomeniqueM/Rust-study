# Estudo de Rush

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

## Rodar o código 

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