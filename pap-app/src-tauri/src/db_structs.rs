use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Autor {
    pub id: i32,
    pub nome: String,
    pub nacionalidade: Option<String>,
    pub data_nasc: Option<String>,
    pub data_morte: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Bibliotecario {
    pub id: i32,
    pub nome: String,
    pub password: String,
    pub cargo: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Categoria {
    pub id: i32,
    pub nome: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Editora {
    pub id: i32,
    pub nome: String,
    pub morada: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Leitor {
    pub id: i32,
    pub nome: String,
    pub morada: String,
    pub telefone: String,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Livro {
    pub id: i32,
    pub nome: String,
    pub resumo: Option<String>,
    pub n_paginas: i32,
    pub idioma: String,
    pub img_url: Option<String>,
    pub ano_edicao: Option<String>,
    pub id_autor: Option<i32>,
    pub id_editora: i32,
    pub id_secao: i32,
    pub id_sub_categoria: Option<i32>,
}

#[derive(Copy, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Requisicao {
    pub id: i32,
    pub id_leitor: i32,
    pub id_livro_requisitado: i32,
    pub data_requisicao: chrono::DateTime<chrono::Utc>,
    pub data_entrega: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Secao {
    pub id: i32,
    pub id_categoria: i32,
    pub nome: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct SubCategoria {
    pub id: i32,
    pub id_categoria: i32,
    pub nome: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Permissao {
    pub id: i32,
    pub acao: String,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct LivroAsResponse {
    pub id: i32,
    pub nome: String,
    pub resumo: Option<String>,
    pub n_paginas: i32,
    pub idioma: String,
    pub img_url: Option<String>,
    pub ano_edicao: Option<String>,
    pub autor: Option<String>,
    pub autor_id: Option<i32>,
    pub editora: String,
    pub id_secao: i32,
    pub categoria: Option<String>,
    pub sub_categoria: Option<String>,
    pub requisitado: bool,
}