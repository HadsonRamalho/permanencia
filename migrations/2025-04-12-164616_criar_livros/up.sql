CREATE TABLE livros(
    id INTEGER PRIMARY KEY,
    nome VARCHAR(64) NOT NULL,
    autor VARCHAR(64) NOT NULL,
    ano_publicacao INTEGER NOT NULL,
    data_cadastro TIMESTAMP NOT NULL,
    data_atualizacao TIMESTAMP
);