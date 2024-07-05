use tokio::sync::Mutex;

use crate::{
    db_structs::{Leitor, Livro, Requisicao},
    jwt::verify_jwt,
    Database,
};

#[tauri::command]
pub async fn request_book(
    token: String,
    book_id: i32,
    reader_id: i32,
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

    let reader = sqlx::query_as::<_, Leitor>("SELECT * FROM leitores WHERE id = ?")
        .bind(reader_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if reader.is_empty() {
        return Err("Leitor não encontrado".to_string());
    }

    let book = sqlx::query_as::<_, Livro>("SELECT * FROM livros WHERE id = ?")
        .bind(book_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if book.is_empty() {
        return Err("Livro não encontrado".to_string());
    }

    sqlx::query("INSERT INTO requisicoes (id_leitor, id_livro_requisitado, data_requisicao) VALUES (?, ?, NOW())")
        .bind(reader_id)
        .bind(book_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao requisitar livro: {}", e);
            format!("Falha ao requisitar livro: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn get_requested_book_by_book_id(
    token: String,
    book_id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(Livro, Requisicao), String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let book = sqlx::query_as::<_, Livro>("SELECT * FROM livros WHERE id = ?")
        .bind(book_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if book.is_empty() {
        return Err("Livro não encontrado".to_string());
    }

    let request =
        sqlx::query_as::<_, Requisicao>("SELECT * FROM requisicoes WHERE id_livro_requisitado = ?")
            .bind(book_id)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar: {}", e);
                format!("Falha ao consultar: {}", e)
            })?;

    if request.is_empty() {
        return Err("Requisição não encontrada".to_string());
    }

    Ok((book[0].clone(), request[0]))
}

#[tauri::command]
pub async fn return_book(
    token: String,
    book_id: i32,
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

    let request = sqlx::query_as::<_, Requisicao>(
        "SELECT * FROM requisicoes WHERE id_livro_requisitado = ? AND data_entrega IS NULL",
    )
    .bind(book_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar: {}", e);
        format!("Falha ao consultar: {}", e)
    })?;

    if request.is_empty() {
        return Err("Requisição não encontrada".to_string());
    }

    sqlx::query("UPDATE requisicoes SET data_entrega = NOW() WHERE id_livro_requisitado = ? AND data_entrega IS NULL")
        .bind(book_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao devolver livro: {}", e);
            format!("Falha ao devolver livro: {}", e)
        })?;

    Ok(())
}
