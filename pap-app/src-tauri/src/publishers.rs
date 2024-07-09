use tokio::sync::Mutex;

use crate::{
    db_structs::{Editora, Livro, LivroAsResponse},
    jwt::verify_jwt,
    Database,
};

#[tauri::command]
pub async fn get_publisher_by_id(
    token: String,
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Editora, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let publisher = sqlx::query_as::<_, Editora>("SELECT * FROM editoras WHERE id = ?")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if publisher.is_empty() {
        return Err("Editora não encontrada".to_string());
    }

    Ok(publisher[0].clone())
}

#[tauri::command]
pub async fn get_publishers_by_name(
    name: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Editora>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let publishers =
        sqlx::query_as::<_, Editora>("SELECT * FROM editoras WHERE LOWER(nome) LIKE LOWER(?)")
            .bind(format!("%{}%", name.to_lowercase()))
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;

    Ok(publishers)
}

#[tauri::command]
pub async fn get_books_by_publisher_id(
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

    let books = sqlx::query_as::<_, Livro>("SELECT * FROM livros WHERE id_editora = ?")
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
pub async fn get_publishers(
    token: String,
    limit: i32,
    offset: i32,
    search: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Editora>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let publishers;

    if search.is_none() {
        publishers = sqlx::query_as::<_, Editora>("SELECT * FROM editoras LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;
    } else {
        publishers = sqlx::query_as::<_, Editora>(
            "SELECT * FROM editoras WHERE LOWER(nome) LIKE LOWER(?) LIMIT ? OFFSET ?",
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

    Ok(publishers)
}

#[tauri::command]
pub async fn get_publishers_count(
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
        count = sqlx::query_scalar("SELECT COUNT(*) FROM editoras")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar autores: {}", e);
                format!("Falha ao consultar autores: {}", e)
            })?;
    } else {
        count = sqlx::query_scalar("SELECT COUNT(*) FROM editoras WHERE LOWER(nome) LIKE LOWER(?)")
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
pub async fn create_publisher(
    token: String,
    name: String,
    address: String,
    postal_code: String,
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

    sqlx::query("INSERT INTO editoras (nome, morada, codigo_postal, telefone, email) VALUES (?, ?, ?, ?, ?)")
        .bind(name)
        .bind(address)
        .bind(postal_code)
        .bind(phone)
        .bind(email)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao criar editora: {}", e);
            format!("Falha ao criar editora: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn update_publisher(
    token: String,
    id: i32,
    name: String,
    address: String,
    postal_code: String,
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

    sqlx::query("UPDATE editoras SET nome = ?, morada = ?, codigo_postal = ?, telefone = ?, email = ? WHERE id = ?")
        .bind(name)
        .bind(address)
        .bind(postal_code)
        .bind(phone)
        .bind(email)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao atualizar editora: {}", e);
            format!("Falha ao atualizar editora: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_publisher(
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

    sqlx::query("DELETE FROM editoras WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao deletar editora: {}", e);
            format!("Falha ao deletar editora: {}", e)
        })?;

    Ok(())
}
