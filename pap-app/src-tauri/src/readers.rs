use tokio::sync::Mutex;

use crate::{db_structs::Leitor, jwt::verify_jwt, Database};

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

    let reader_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM leitores WHERE email = ?")
        .bind(&email)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if reader_count > 0 {
        return Err("Leitor já existe".to_string());
    }

    sqlx::query("INSERT INTO leitores (nome, morada, telefone, email) VALUES (?, ?, ?, ?)")
        .bind(name)
        .bind(address)
        .bind(phone)
        .bind(email)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao inserir leitor: {}", e);
            format!("Falha ao inserir leitor: {}", e)
        })?;

    Ok(())
}
