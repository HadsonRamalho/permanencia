use axum::Router;
use dotenvy::dotenv;
use std::env;
use diesel::prelude::*;

const PORTA:u32 = 3030;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let uri: &str = &format!("0.0.0.0:{}", PORTA);
    let listener = tokio::net::TcpListener::bind(uri).await.unwrap();
    
    println!("API rodando em {}", uri);

    axum::serve(listener, app).await.unwrap();
}

pub fn criar_conexao() -> PgConnection{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("A URL do banco deveria estar presente no arquivo .env.");
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_|panic!("Erro ao conectar ao banco."))
}