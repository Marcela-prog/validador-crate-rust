# 🦀 Validador CPF

Uma crate desenvolvida em Rust para validar números de CPF (Cadastro de Pessoas Físicas) utilizando o algoritmo oficial dos dígitos verificadores.

Este projeto foi desenvolvido durante os estudos sobre **Crates**, **modularização**, **namespaces** e **publicação de bibliotecas em Rust**.

---

## ✨ Funcionalidades

* ✅ Validação de CPF utilizando o algoritmo oficial
* ✅ Aceita CPF com ou sem formatação
* ✅ Rejeita CPFs com todos os dígitos iguais
* ✅ API simples e fácil de utilizar
* ✅ Sem dependências externas

---

## 📂 Estrutura do projeto

```
validador-crate-rust
│
├── src
│   └── lib.rs
│
├── Cargo.toml
├── Cargo.lock
└── .gitignore
```

---

## 📦 Instalação

Adicione a crate no arquivo `Cargo.toml`:

```toml
[dependencies]

# Busca diretório local
validador_crate_rust_ma = { path = "../validador" }

# Busca pelo GitHub
validador_crate_rust_ma = { git = "https://github.com/Marcela-prog/validador-crate-rust.git" }

# Busca pelo crates.io
validador_crate_rust_ma = "1.0.0"
```

Depois execute:

```bash
cargo build
```

---

## 🚀 Exemplo de uso

```rust
use validador_crate_rust_ma as vd;
use std::io;

fn main() {

    println!("Digite um CPF para validar (sem pontuação)");

    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("Você digitou: {}", cpf.trim());
        }

        Err(e) => {
            println!("Erro ao ler a entrada: {}", e);
            return;
        }
    }

    let validado = vd::validador::cpf(cpf.trim());
    //let validado = vd::validador::cnpj( cnpj.trim()); ou CNPJ

    if validado {
        println!("O CPF é válido.");
    } else {
        println!("O CPF é inválido.");
    }
}
```

---

## 📖 API

### validar_cpf()

```rust
pub fn validar_cpf(cpf: &str) -> bool
```

Recebe uma string contendo um CPF e retorna:

| Retorno | Significado  |
| ------- | ------------ |
| `true`  | CPF válido   |
| `false` | CPF inválido |

---

## ✔ Regras de validação

A função realiza as seguintes verificações:

* Remove caracteres de formatação automaticamente;
* Verifica se existem exatamente 11 dígitos;
* Rejeita sequências com todos os dígitos iguais;
* Calcula e valida os dois dígitos verificadores conforme o algoritmo oficial da Receita Federal.

---

## 🛠 Tecnologias utilizadas

* Rust
* Cargo

---

## 📚 Conteúdos estudados

Este projeto foi desenvolvido durante os estudos de:

* Modularização com Crates
* Criação de bibliotecas Rust
* Dependências locais
* Dependências via GitHub
* Organização de código com Namespaces
* Publicação de Crates

---

## 🚀 Próximas melhorias

* [ ] Publicação no crates.io
* [ ] Documentação utilizando rustdoc
* [ ] Testes automatizados
* [ ] CI com GitHub Actions

---

## 👩‍💻 Autora

**Marcela Nogueira**

* GitHub: https://github.com/Marcela-prog
* LinkedIn: "https://www.linkedin.com/in/marcela-nogueira-855272191"

---

⭐ Se este projeto foi útil para você, deixe uma estrela no repositório!

