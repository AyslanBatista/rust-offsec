# 🦀 Rust Offsec

<div align="center">
  <img src="assets/capa_rust.png" alt="Capa do Rust Offsec" width="300"/>
  <p><strong>Ferramentas de Segurança Ofensiva Implementadas em Rust</strong></p>
  
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![Security Tools](https://img.shields.io/badge/Security-Tools-red?style=for-the-badge)](https://github.com/AyslanBatista/rust-offsec)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
</div>

## 📋 Sobre o Projeto

Este repositório documenta minha jornada de aprendizado em segurança cibernética utilizando a linguagem Rust. Aqui você encontrará implementações de exploits, scripts de automação e ferramentas para pentesting que desenvolvo enquanto estudo e participo de CTFs (Capture The Flag).


## 🗂️ Estrutura do Repositório

### 💀 [Exploits](https://github.com/AyslanBatista/rust-offsec/tree/main/exploits)

Implementações de exploits conhecidos, reescritos em Rust para estudo e prática.

<details>
  <summary><b>Exploits Disponíveis</b></summary>
  
  | CVE/Nome | Descrição | Alvo |
  |----------|-----------|------|
  | [CVE-2018-16763](https://github.com/AyslanBatista/rust-offsec/tree/main/exploits/CVE-2018-16763) | Execução remota de código no CMS Made Simple | CMS Made Simple <= 2.2.5 |
  | [CVE-2018-19422](https://github.com/AyslanBatista/rust-offsec/tree/main/exploits/CVE-2018-19422) | Vulnerabilidade de injeção SQL | DVWA |
  | [CVE-2023-27040](https://github.com/AyslanBatista/rust-offsec/tree/main/exploits/CVE-2023-27040) | Vulnerabilidade RCE via upload de arquivo | Apache Superset <= 2.0.1 |
  | [php-8_1_0-dev-backdoor-rce](https://github.com/AyslanBatista/rust-offsec/tree/main/exploits/php-8_1_0-dev-backdoor-rce) | Exploit para backdoor em versão de desenvolvimento | PHP 8.1.0-dev |
  | [osCommerce-2_3_4-rce](https://github.com/AyslanBatista/rust-offsec/tree/main/exploits/osCommerce-2_3_4-rce) | Exploração de RCE através de instalação não protegida | osCommerce 2.3.4 |
</details>

### 🤖 [Scripts](https://github.com/AyslanBatista/rust-offsec/tree/main/scripts)

Scripts para automação de tarefas comuns em pentests e CTFs, otimizados para performance.

<details>
  <summary><b>Scripts Disponíveis</b></summary>
  
  | Nome | Descrição | Uso |
  |------|-----------|-----|
  | [check-valid-emails-THM](https://github.com/AyslanBatista/rust-offsec/tree/main/scripts/check-valid-emails-THM) | Verificador de emails válidos | TryHackMe challenges |
  | [brute-force-hammerTHM](https://github.com/AyslanBatista/rust-offsec/tree/main/scripts/brute-force-hammerTHM) | Script de força bruta otimizado | TryHackMe Hammer lab |
  | [blind-ldap-extraction-THM](https://github.com/AyslanBatista/rust-offsec/tree/main/scripts/blind-ldap-extraction-THM) | Extração de dados via injeção LDAP cega | TryHackMe LDAP challenges |
</details>

### 🛠️ [Ferramentas](https://github.com/AyslanBatista/rust-offsec/tree/main/tools)

Utilitários e ferramentas standalone para testes de segurança.

<details>
  <summary><b>Ferramentas Disponíveis</b></summary>
  
  | Nome | Descrição | Recursos |
  |------|-----------|----------|
  | [nosql_password_checker](https://github.com/AyslanBatista/rust-offsec/tree/main/tools/nosql_password_checker) | Ferramenta para testar vulnerabilidades NoSQL injection | Suporte para MongoDB, verificação de bypass de autenticação |
</details>
