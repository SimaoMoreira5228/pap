use tokio::sync::Mutex;

use crate::{
    db_structs::{Autor, Livro, LivroAsResponse},
    jwt::verify_jwt,
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
pub async fn get_authors_by_name(
    name: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Autor>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let authors =
        sqlx::query_as::<_, Autor>("SELECT * FROM autores WHERE LOWER(nome) LIKE LOWER(?)")
            .bind(format!("%{}%", name.to_lowercase()))
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;

    Ok(authors)
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
            n_paginas: book.n_paginas,
            requisitado: is_requested > 0,
        });
    }

    Ok(respose_books)
}

#[tauri::command]
pub async fn get_authors(
    token: String,
    limit: i32,
    offset: i32,
    search: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Autor>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let authors;

    if search.is_none() {
        authors = sqlx::query_as::<_, Autor>("SELECT * FROM autores LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;
    } else {
        authors = sqlx::query_as::<_, Autor>(
            "SELECT * FROM autores WHERE LOWER(nome) LIKE LOWER(?) LIMIT ? OFFSET ?",
        )
        .bind(format!("%{}%", search.unwrap().to_lowercase()))
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar autores: {}", e);
            format!("Falha ao consultar autores: {}", e)
        })?;
    }

    Ok(authors)
}

#[tauri::command]
pub async fn get_authors_count(
    token: String,
    search: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<i32, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let count: i32;

    if search.is_none() {
        count = sqlx::query_scalar("SELECT COUNT(*) FROM autores")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;
    } else {
        count = sqlx::query_scalar("SELECT COUNT(*) FROM autores WHERE LOWER(nome) LIKE LOWER(?)")
            .bind(format!("%{}%", search.unwrap().to_lowercase()))
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;
    }

    Ok(count)
}

#[tauri::command]
pub async fn create_author(
    token: String,
    name: String,
    nationality: String,
    birth_date: Option<String>,
    death_date: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(), String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    sqlx::query(
        "INSERT INTO autores (nome, nacionalidade, data_nasc, data_morte) VALUES (?, ?, ?, ?)",
    )
    .bind(name)
    .bind(nationality)
    .bind(birth_date)
    .bind(death_date)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao criar autor: {}", e);
        format!("Falha ao criar autor: {}", e)
    })?;

    Ok(())
}

#[tauri::command]
pub async fn update_author(
    token: String,
    id: i32,
    name: String,
    nationality: String,
    birth_date: Option<String>,
    death_date: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(), String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    sqlx::query("UPDATE autores SET nome = ?, nacionalidade = ?, data_nasc = ?, data_morte = ? WHERE id = ?")
        .bind(name)
        .bind(nationality)
        .bind(birth_date)
        .bind(death_date)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao atualizar autor: {}", e);
            format!("Falha ao atualizar autor: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_author(
    token: String,
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(), String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    sqlx::query("DELETE FROM autores WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao deletar autor: {}", e);
            format!("Falha ao deletar autor: {}", e)
        })?;

    Ok(())
}
