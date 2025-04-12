# Permanência API

Permanência API é uma aplicação web desenvolvida em Rust para gerenciar um acervo de livros. Ela utiliza o framework Axum para a criação de rotas e o Diesel ORM para interações com o banco de dados PostgreSQL.

## Funcionalidades

- **Cadastrar Livro**: Endpoint para adicionar um novo livro ao acervo.
- **Listar Livros**: Endpoint para listar todos os livros cadastrados, ordenados pela data de cadastro.

## Estrutura do Projeto

```
├── src/ 
│   ├── livros.rs       # Lógica para operações na API e no banco de dados
│   ├── main.rs         # Ponto de entrada da aplicação 
│   ├── schema.rs       # Definições do esquema do banco de dados 
├── migrations/         # Diretório de migrações do banco de dados 
├── .env                # Configurações de ambiente (URL do banco de dados) 
├── Cargo.toml          # Configurações e dependências do projeto 
├── diesel.toml         # Configurações do Diesel ORM
```

## Endpoints

### 1. Cadastrar Livro
- **URL**: `/cadastrar_livro`
- **Método**: POST
- **Corpo da Requisição**:
  ```json
  {
    "nome": "Nome do Livro",
    "autor": "Nome do Autor",
    "ano_publicacao": 2025
  }

- **Resposta:**
  ```json 
    "Sucesso": "Livro Cadastrado | ID: <id> | Nome: <nome>",
    "Erro": "Mensagem de erro de validação ou do banco de dados."

## Dependências
- **Axum** - Framework web assíncrono.
- **Diesel** - ORM para interações com o banco de dados.
- **Tokio** - Runtime assíncrono para Rust.
- **Serde** - Serialização e desserialização de dados.