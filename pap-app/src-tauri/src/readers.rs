use tokio::sync::Mutex;

use crate::{
    db_structs::{Leitor, Livro, LivroAsResponse},
    jwt::verify_jwt,
    Database,
};

#[tauri::command]
pub async fn get_reader_by_id(
    token: String,
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Option<Leitor>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let reader = sqlx::query_as::<_, Leitor>("SELECT * FROM leitores WHERE id = ?")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    Ok(reader.first().cloned())
}

#[tauri::command]
pub async fn get_readers_by_name(
    token: String,
    name: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Leitor>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let readers = sqlx::query_as::<_, Leitor>("SELECT * FROM leitores WHERE nome LIKE ?")
        .bind(format!("%{}%", name))
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    Ok(readers)
}

#[tauri::command]
pub async fn get_readers(
    token: String,
    limit: i32,
    offset: i32,
    search: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Leitor>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let readers;

    if search.is_none() {
        readers = sqlx::query_as::<_, Leitor>("SELECT * FROM leitores LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar leitores: {}", e);
                format!("Falha ao consultar leitores: {}", e)
            })?;
    } else {
        readers = sqlx::query_as::<_, Leitor>(
            "SELECT * FROM leitores WHERE LOWER(nome) LIKE LOWER(?) LIMIT ? OFFSET ?",
        )
        .bind(format!("%{}%", search.unwrap().to_lowercase()))
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar leitores: {}", e);
            format!("Falha ao consultar leitores: {}", e)
        })?;
    }

    Ok(readers)
}

#[tauri::command]
pub async fn get_readers_count(
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
        count = sqlx::query_scalar("SELECT COUNT(*) FROM leitores")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar leitores: {}", e);
                format!("Falha ao consultar leitores: {}", e)
            })?;
    } else {
        count = sqlx::query_scalar("SELECT COUNT(*) FROM leitores WHERE LOWER(nome) LIKE LOWER(?)")
            .bind(format!("%{}%", search.unwrap().to_lowercase()))
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar leitores: {}", e);
                format!("Falha ao consultar leitores: {}", e)
            })?;
    }

    Ok(count)
}

#[tauri::command]
pub async fn create_reader(
    token: String,
    name: String,
    address: String,
    phone: String,
    email: String,
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

    sqlx::query("INSERT INTO leitores (nome, morada, telefone, email) VALUES (?, ?, ?, ?)")
        .bind(name)
        .bind(address)
        .bind(phone)
        .bind(email)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao criar leitor: {}", e);
            format!("Falha ao criar leitor: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn update_reader(
    token: String,
    id: i32,
    name: String,
    address: String,
    phone: String,
    email: String,
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

    sqlx::query("UPDATE leitores SET nome = ?, morada = ?, telefone = ?, email = ? WHERE id = ?")
        .bind(name)
        .bind(address)
        .bind(phone)
        .bind(email)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao atualizar leitor: {}", e);
            format!("Falha ao atualizar leitor: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_reader(
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

    sqlx::query("DELETE FROM leitores WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao deletar leitor: {}", e);
            format!("Falha ao deletar leitor: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn get_requested_books_by_reader_id(
    token: String,
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<LivroAsResponse>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let request_books_ids = sqlx::query_scalar::<_, i32>(
        "SELECT id_livro_requisitado FROM requisicoes WHERE id_leitor = ? AND data_entrega IS NULL",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar requisições: {}", e);
        format!("Falha ao consultar requisições: {}", e)
    })?;

    let mut books: Vec<Livro> = Vec::new();

    for book_id in request_books_ids {
        books.push(
            sqlx::query_as::<_, Livro>("SELECT * FROM livros WHERE id = ?")
                .bind(book_id)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Falha ao consultar livro: {}", e);
                    format!("Falha ao consultar livro: {}", e)
                })?,
        );
    }

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
            nome: book.nome,
            resumo: book.resumo,
            n_paginas: book.n_paginas,
            idioma: book.idioma,
            img_url: book.img_url,
            ano_edicao: book.ano_edicao,
            autor: Some(autor),
            autor_id: book.id_autor,
            editora: publisher,
            categoria: Some(categoria),
            sub_categoria: Some(sub_categoria),
            requisitado: is_requested > 0,
        });
    }

    Ok(respose_books)
}
