use axum::{
    routing::{delete, get, patch, post}, Router
};
use dotenvy::dotenv;
use livros::{atualizar_livro, buscar_livro_autor, buscar_livro_categoria, buscar_livro_id, deletar_livro};
use std::env;
use diesel::prelude::*;
use crate::livros::{cadastrar_livro, listar_livros};
// use tower_http::cors::{Any, CorsLayer};
// axum::http::Method
mod livros;
pub mod schema;

const PORTA:u32 = 3030;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/cadastrar_livro", post(cadastrar_livro))
        .route("/listar_livros", get(listar_livros))
        .route("/atualizar_livro", patch(atualizar_livro))
        .route("/deletar_livro/", delete(deletar_livro))
        .route("/buscar_livro_id/", get(buscar_livro_id))
        .route("/buscar_livro_autor/", get(buscar_livro_autor))
        .route("/buscar_livro_categoria/", get(buscar_livro_categoria));

    /*
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::GET]) 
            .allow_headers(Any)
    )
    */

    let uri: &str = &format!("0.0.0.0:{}", PORTA);
    let listener = tokio::net::TcpListener::bind(uri).await.unwrap();
    
    println!("API rodando em http://localhost:3030/ (ou {})", uri);

    axum::serve(listener, app).await.unwrap();
}

pub fn criar_conexao() -> PgConnection{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("A URL do banco deveria estar presente no arquivo .env.");
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_|panic!("Erro ao conectar ao banco."))
}