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

## Escrevendo código em Rust

A extersão usada para os arquivos rust é:  __.rs__ 

### Rodar o código 

```bash
rustc nomeDoArquivo.rs
```