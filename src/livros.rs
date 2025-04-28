use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use axum::{extract::Query, http::StatusCode, Json};

use crate::criar_conexao;

#[derive(Serialize, Deserialize)]
pub struct LivroInput{
    nome: String,
    autor: String,
    ano_publicacao: i32,
    categorias: Vec<Option<String>>
}

#[derive(Serialize, Deserialize)]
pub struct AtualizarLivroInput{
    pub id: i32,
    pub nome: String,
    pub autor: String,
    pub ano_publicacao: i32,
    pub categorias: Vec<Option<String>>
}

#[derive(Queryable, Selectable, Insertable, Debug, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::livros)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Livro{
    id: i32,
    nome: String,
    autor: String,
    ano_publicacao: i32,
    data_cadastro: NaiveDateTime,
    data_atualizacao: Option<NaiveDateTime>,
    categorias: Vec<Option<String>>
}

impl From<LivroInput> for Livro{
    fn from(input: LivroInput) -> Self{
        let id:u16 = rand::random();
        Self{
            id: id as i32,
            nome: input.nome,
            autor: input.autor,
            ano_publicacao: input.ano_publicacao as i32,
            data_atualizacao: None,
            data_cadastro: chrono::Utc::now().naive_utc(),
            categorias: input.categorias
        }
    }
}

impl LivroInput{
    pub fn validar_campos(&self) 
        -> Result<(), Json<String>>{
        let mut campos_faltando = vec![];

        if self.ano_publicacao.to_string().trim().is_empty(){
            campos_faltando.push("Ano de Publicação");
        }

        if self.nome.trim().is_empty(){
            campos_faltando.push("Nome");
        }

        if self.autor.trim().is_empty(){
            campos_faltando.push("Autor");
        }

        if self.categorias.is_empty(){
            campos_faltando.push("Categorias");
        }
        
        if campos_faltando.len() > 0{
            return Err(Json(format!("Erro ao cadastrar o livro. Os campos [{}] deveriam estar preenchidos.", campos_faltando.join(", "))));
        }
        return Ok(())
    }
}

pub async fn cadastrar_livro(input: Json<LivroInput>)
    -> Result<Json<String>, Json<String>>{
    use crate::schema::livros::dsl::*;
    if let Err(mensagem_erro) = input.0.validar_campos() {
        return Err(mensagem_erro);
    }

    let livro = input.0;
    
    let livro = Livro::from(livro);

    println!("Dados do livro: {:?}", livro);

    let mensagem_retorno = format!("Livro Cadastrado | ID: {} | Nome: {}", livro.id, &livro.nome);

    let conexao = &mut criar_conexao();

    let res = diesel::insert_into(livros)
        .values(livro)
        .get_result::<Livro>(conexao);
    
    match res{
        Ok(_livro) => {
            return Ok(Json(mensagem_retorno))
        },
        Err(e) => {
            return Err(Json(e.to_string()))
        }
    }

}

pub async fn listar_livros()
    -> Result<Json<Vec<Livro>>, Json<String>>{
    use crate::schema::livros::dsl::*;

    let conexao = &mut criar_conexao();

    let res = livros
        .order_by(data_cadastro.desc())
        .get_results::<Livro>(conexao);

    match res{
        Ok(veclivros) => {
            return Ok(Json(veclivros))
        },
        Err(e) => {
            return Err(Json(e.to_string()))
        }
    }

}

pub async fn atualizar_livro(input: Json<AtualizarLivroInput>)
    -> Result<Json<Livro>, Json<String>>{
    use crate::schema::livros::dsl::*;

    let conexao = &mut criar_conexao();

    let livro = input.0;

    let res: Result<Livro, diesel::result::Error> = diesel::update(livros)
        .set((
            nome.eq(livro.nome), 
            autor.eq(livro.autor),
            ano_publicacao.eq(livro.ano_publicacao),
            categorias.eq(livro.categorias)
        ))
        .filter(id.eq(livro.id))
        .get_result(conexao);

    match res{
        Ok(livro) => {
            return Ok(Json(livro))
        },
        Err(e) => {
            return Err(Json(e.to_string()))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IdInput{
    id: i32
}

pub async fn deletar_livro(Query(input): Query<IdInput>)
    -> Result<(StatusCode, Json<Livro>), (StatusCode, Json<String>)>{
    use crate::schema::livros::dsl::*;

    let conexao = &mut criar_conexao();

    let res: Result<Livro, diesel::result::Error> = diesel::delete(livros)
        .filter(id.eq(input.id))
        .get_result(conexao);

    match res{
        Ok(livro_deletado) => {
            return Ok((StatusCode::OK, Json(livro_deletado)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }
}

pub async fn buscar_livro_id(Query(input): Query<IdInput>)
    -> Result<(StatusCode, Json<Livro>), (StatusCode, Json<String>)>{
    use crate::schema::livros::dsl::*;

    let conexao = &mut criar_conexao();

    let res: Result<Livro, diesel::result::Error> = livros
        .filter(id.eq(input.id))
        .get_result(conexao);

    match res{
        Ok(livro) => {
            return Ok((StatusCode::OK, Json(livro)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct AutorInput{
    autor: String
}

pub async fn buscar_livro_autor(Query(input): Query<AutorInput>)
    -> Result<(StatusCode, Json<Vec<Livro>>), (StatusCode, Json<String>)>{
    use crate::schema::livros::dsl::*; 

    let conexao = &mut criar_conexao();

    let nome_autor = input.autor.to_string();
    let nome_autor = format!("%{}%", nome_autor);

    let res: Result<Vec<Livro>, diesel::result::Error> = livros
        .filter(autor.like(nome_autor))
        .get_results(conexao);

    match res{
        Ok(livro) => {
            return Ok((StatusCode::OK, Json(livro)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CategoriaInput{
    categoria: String
}

pub async fn buscar_livro_categoria(Query(input): Query<CategoriaInput>)
    -> Result<(StatusCode, Json<Vec<Livro>>), (StatusCode, Json<String>)>{
    use crate::schema::livros::dsl::*; 

    let conexao = &mut criar_conexao();

    let categoria = input.categoria.to_string();
    use diesel::sql_query;
    use diesel::RunQueryDsl;
    
    let query = sql_query("
        SELECT * FROM livros 
        WHERE EXISTS (
            SELECT 1 FROM unnest(categorias) AS cat WHERE cat LIKE $1
        )
    ")
    .bind::<diesel::sql_types::Text, _>(format!("%{}%", categoria));
    
    let results: Result<Vec<Livro>, diesel::result::Error> = query.load(conexao);

    let results = match results {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    };

    return Ok((StatusCode::OK, Json(results)))

}

