# Carteira de Investimentos Fullstack com Rust 💼🦀

Esta é a minha resolução oficial para o desafio de projeto desenvolvido através da **Digital Innovation One (DIO)**. O projeto consiste em construir e consolidar uma aplicação Fullstack utilizando o ecossistema da linguagem Rust.

---

## 🔎 O que o projeto faz

A aplicação funciona como um sistema centralizado de gerenciamento de carteira de investimentos. Com ela, o usuário consegue registrar novos ativos, lançar e listar movimentações financeiras de compra ou venda e acompanhar a consolidação total de seu patrimônio através de uma interface web.

---

## 🛠️ Tecnologias Utilizadas

* **Linguagem Backend:** Rust (focada em alta performance e segurança de memória)
* **Framework Web & API:** Axum / Actix-web (Gerenciamento de rotas HTTP e endpoints)
* **Renderização de Views:** Tera Templates / HTML Estático (Camada de Front-end integrada)
* **Banco de Dados:** PostgreSQL (Persistência dos dados financeiros)
* **Ferramenta de Migrações:** SQLx / Refinery (Execução automática dos scripts da pasta `migrations/`)
* **Containerização:** Docker e Docker Compose (`compose.yml`)

---

## 📦 Como Executar a Aplicação

Siga os passos abaixo para preparar o ambiente e rodar o projeto localmente.

### Pré-requisitos
* **Rust & Cargo** instalados via `rustup`.
* **Docker** e **Docker Compose** ativos na máquina.

### Passo a Passo

1. **Clone o repositório:**
   ```bash
   git clone https://github.com
   cd rust-fullstack-carteira-investimentos
   ```

2. **Configuração de Ambiente:**
   * O projeto utiliza o arquivo `.env` para carregar as credenciais da `DATABASE_URL`. Certifique-se de que os dados apontam para o container configurado.

3. **Subir o Banco de Dados (Docker):**
   ```bash
   docker compose up -d
   ```

4. **Executar a Aplicação:**
   ```bash
   cargo run
   ```
   O compilador baixará as dependências e iniciará o servidor. Assim que finalizado, acesse o painel pelo navegador em `http://localhost:8080`.

---

## 🚀 Melhoria Implementada

Como parte dos requisitos de evolução técnica propostos pelo desafio, foi desenvolvida a seguinte alteração:
* **Validação e Filtros de Segurança de Ativos:** Refatoração e inclusão de validações básicas nas regras de negócio da aplicação para impedir o cadastro de movimentações financeiras inconsistentes (como valores negativos ou campos em branco) e melhorias na listagem da tabela principal.

---

## 🧪 Como Testar a Minha Versão

Para validar as alterações aplicadas nesta versão do projeto:
1. Acesse o painel web no seu navegador através do endereço `http://localhost:8080`.
2. Vá até a seção de lançamentos ou gerenciamento de ativos.
3. Tente inserir uma nova movimentação e valide se as informações estão sendo computadas e refletidas corretamente nos cards de resumo do saldo consolidado.

---

## 📖 Aprendizados Obtidos

* **Garantias do Compilador Rust:** Compreensão prática sobre o funcionamento das regras rígidas do *Borrow Checker* e tratamento seguro de erros com os enums `Result` e `Option`.
* **Conexão Assíncrona com Bancos de Dados:** Integração do ecossistema Rust com o PostgreSQL, lidando com conexões assíncronas e execução de migrações estruturadas.
* **Arquitetura Fullstack Monolítica:** Entendimento de como o ecossistema Rust atua unificado gerenciando tanto as requisições de API quanto servindo arquivos e templates para a interface visual do usuário.

---
Desenvolvido por **Maicon** durante o Desafio Fullstack da DIO. 🚀
