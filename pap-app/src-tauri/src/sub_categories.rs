use tokio::sync::Mutex;

use crate::{db_structs::SubCategoria, jwt::verify_jwt, Database};

#[tauri::command]
pub async fn get_sub_categories(
    token: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<SubCategoria>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let sub_categories = sqlx::query_as::<_, SubCategoria>("SELECT * FROM sub_categorias")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar sub-categorias: {}", e);
            format!("Falha ao consultar sub-categorias: {}", e)
        })?;

    Ok(sub_categories)
}

#[tauri::command]
pub async fn get_sub_categories_by_name(
    name: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<SubCategoria>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let sub_categories = sqlx::query_as::<_, SubCategoria>(
        "SELECT * FROM sub_categorias WHERE LOWER(nome) LIKE LOWER(?)",
    )
    .bind(format!("%{}%", name.to_lowercase()))
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar sub-categorias: {}", e);
        format!("Falha ao consultar sub-categorias: {}", e)
    })?;

    Ok(sub_categories)
}

#[tauri::command]
pub async fn delete_sub_category(
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

    let used = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT * FROM livros WHERE id_sub_categoria = ?)",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar se sub-categoria é usada: {}", e);
        format!("Falha ao consultar se sub-categoria é usada: {}", e)
    })?;

    if used {
        return Err(
            "Esta sub-categoria tem livros associados, não é possível apagá-la".to_string(),
        );
    }

    sqlx::query("DELETE FROM sub_categorias WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao deletar sub-categoria: {}", e);
            format!("Falha ao deletar sub-categoria: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn create_sub_category(
    token: String,
    name: String,
    category_id: i32,
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

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT * FROM sub_categorias WHERE nome = ? AND id_categoria = ?)",
    )
    .bind(&name)
    .bind(category_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar se sub-categoria já existe: {}", e);
        format!("Falha ao consultar se sub-categoria já existe: {}", e)
    })?;

    if exists {
        return Err("Esta sub-categoria já existe".to_string());
    }

    sqlx::query("INSERT INTO sub_categorias (nome, id_categoria) VALUES (?, ?)")
        .bind(&name)
        .bind(category_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao inserir sub-categoria: {}", e);
            format!("Falha ao inserir sub-categoria: {}", e)
        })?;

    Ok(())
}
