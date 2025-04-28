// @generated automatically by Diesel CLI.

diesel::table! {
    livros (id) {
        id -> Int4,
        #[max_length = 64]
        nome -> Varchar,
        #[max_length = 64]
        autor -> Varchar,
        ano_publicacao -> Int4,
        data_cadastro -> Timestamp,
        data_atualizacao -> Nullable<Timestamp>,
        categorias -> Array<Nullable<Text>>,
    }
}
