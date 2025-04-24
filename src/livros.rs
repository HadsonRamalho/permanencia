use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use axum::Json;

use crate::criar_conexao;

#[derive(Serialize, Deserialize)]
pub struct LivroInput{
    nome: String,
    autor: String,
    ano_publicacao: u32
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
    data_atualizacao: Option<NaiveDateTime>
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
            data_cadastro: chrono::Utc::now().naive_utc()
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
        
        if campos_faltando.len() > 0{
            return Err(Json(format!("Erro ao cadastrar o livro. Os campos [{}] deveriam estar preenchidos.", campos_faltando.join(", "))));
        }
        return Ok(())
    }
}

pub async fn cadastrar_livro(input: Json<LivroInput>)
    -> Result<Json<String>, Json<String>>{
    use crate::schema::livros::dsl::*;
    let v = input.0;
    if let Err(mensagem_erro) = v.validar_campos() {
        return Err(mensagem_erro);
    }
    
    let livro = Livro::from(v);

    println!("Dados do livro: {:?}", livro);

    let mensagem_retorno = format!("Livro Cadastrado | ID: {} | Nome: {}", livro.id, &livro.nome);

    let conexao = &mut criar_conexao();

    let res = diesel::insert_into(livros)
        .values(livro)
        .get_result::<Livro>(conexao);
    
    match res{
        Ok(livro) => {
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