use tokio::sync::Mutex;

use crate::{
    db_structs::{Autor, Livro, LivroAsResponse},
    Database,
};

#[tauri::command]
pub async fn get_author_by_id(
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Autor, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let author = sqlx::query_as::<_, Autor>("SELECT * FROM autores WHERE id = ?")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if author.is_empty() {
        return Err("Autor não encontrado".to_string());
    }

    Ok(author[0].clone())
}

#[tauri::command]
pub async fn get_books_by_author_id(
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<LivroAsResponse>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let books = sqlx::query_as::<_, Livro>("SELECT * FROM livros WHERE id_autor = ?")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    let mut respose_books: Vec<LivroAsResponse> = Vec::new();

    for book in books {
        let autor = sqlx::query_scalar::<_, String>("SELECT nome FROM autores WHERE id = ?")
            .bind(book.id_autor)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autor: {}", e);
                format!("Falha ao consultar autor: {}", e)
            })?;

        let categoria = sqlx::query_scalar::<_, String>(
            "SELECT nome FROM categorias WHERE id = (SELECT id_categoria FROM sub_categorias WHERE id = ?)",
        )
        .bind(book.id_sub_categoria)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar categoria: {}", e);
            format!("Falha ao consultar categoria: {}", e)
        })?;

        let sub_categoria =
            sqlx::query_scalar::<_, String>("SELECT nome FROM sub_categorias WHERE id = ?")
                .bind(book.id_sub_categoria)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Falha ao consultar sub-categoria: {}", e);
                    format!("Falha ao consultar sub-categoria: {}", e)
                })?;

        let publisher = sqlx::query_scalar::<_, String>("SELECT nome FROM editoras WHERE id = ?")
            .bind(book.id_editora)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar editora: {}", e);
                format!("Falha ao consultar editora: {}", e)
            })?;

        let is_requested: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM requisicoes WHERE id_livro_requisitado = ? AND data_entrega IS NULL",
    )
    .bind(book.id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar requisição: {}", e);
        format!("Falha ao consultar requisição: {}", e)
    })?;

        respose_books.push(LivroAsResponse {
            id: book.id,
            autor: Some(autor),
            categoria: Some(categoria),
            sub_categoria: Some(sub_categoria),
            ano_edicao: book.ano_edicao,
            img_url: book.img_url,
            resumo: book.resumo,
            autor_id: book.id_autor,
            editora: publisher,
            idioma: book.idioma,
            nome: book.nome,
            id_secao: book.id_secao,
            n_paginas: book.n_paginas,
            requisitado: is_requested > 0,
        });
    }

    Ok(respose_books)
}
