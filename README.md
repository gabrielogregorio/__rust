#   get started

https://www.rust-lang.org/pt-BR/learn/get-started


| description | command |
|-------------------------|-------------|
| new | cargo new project-name |
| compile | cargo build |
| run | cargo run |
| test | cargo test |
| make docs | cargo doc |


## How use cargo-watch
```bash
cargo install cargo-watch
cargo-watch -x run
```

## Running script files

```bash
cargo install cargo-play
cargo-play src/main.rs 
```

## Release version without dev code 
```bash
cargo build --release
```

## Create libs
```bash
cargo new mylib --lib
```
