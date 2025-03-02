# Hammer THM Password Reset Brute-Force

Este script Rust implementa um ataque de força bruta automatizado para a funcionalidade de redefinição de senha do alvo "hammer.thm".


## Uso

Execute o script com:
```bash
cargo run
```

O script irá:
- Obter um PHPSESSID inicial
- Testar códigos de recuperação de 0000 a 9999
- Renovar o PHPSESSID a cada 7 tentativas
- Mostrar o progresso durante a execução
- Exibir o código encontrado e o tempo total ao finalizar
