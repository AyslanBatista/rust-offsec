# Blind LDAP Extraction Script em Rust

## Descrição
Script em Rust para extração de informações através de uma técnica de SQL LDAP Injection. O script tenta extrair caracteres de um e-mail de forma incremental, testando caractere por caractere em uma requisição HTTP POST.

## Funcionalidades
- Realiza requisições POST para uma URL específica
- Tenta extrair caracteres de um email desconhecido
- Usa uma estratégia de força bruta com conjunto predefinido de caracteres
- Imprime caracteres descobertos progressivamente

## Dependências
- reqwest
- tokio
- scraper

## Uso
Execute o script com:
```bash
cargo run
```

