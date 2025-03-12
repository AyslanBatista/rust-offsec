# NoSQL Password Checker
![](../../assets/nosql_password_checker.gif)
Uma ferramenta para CTF que explora vulnerabilidades NoSQL Injection para descobrir senhas em aplicações web.

## Sobre

Esta ferramenta automatiza o processo de exploração de vulnerabilidades de injeção NoSQL em formulários de login. Ela usa técnicas de regex para determinar o tamanho da senha e, em seguida, encontra cada caractere através de um processo de força bruta.

## Funcionalidades

- Teste de requisições HTTP com configurações personalizáveis
- Descoberta automática do tamanho da senha
- Brute force de senhas através de NoSQL Injection
- Suporte para diferentes tipos de payloads (form, json, raw)

## Instalação

```bash
# Clone o repositório
git clone https://github.com/AyslanBatista/rust-offsec.git
cd rust-offsec/tools/nosql_password_checker/

# Compile o projeto
cargo build --release
```

## Uso

### Configuração

Edite o arquivo `config/default.toml` para configurar:
- Host, protocolo e caminho alvo
- Método HTTP e headers
- Payload com a vulnerabilidade NoSQL
- Padrões de resposta para sucesso/erro

### Exemplo de Payload

```
user=admin&pass[$regex]=^.{11}$&remember=on
```

### Comandos

```bash
# Fazer uma requisição simples
./target/release/nosql_password_checker request

# Descobrir uma senha através de brute force
./target/release/nosql_password_checker bruteforce

# Usar um host diferente
./target/release/nosql_password_checker bruteforce -H exemplo.com

# Ver ajuda
./target/release/nosql_password_checker --help
```

## Dependências

- Rust 2024 ou superior
- reqwest, regex, clap e outras bibliotecas (veja Cargo.toml)
