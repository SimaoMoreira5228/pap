use tokio::sync::Mutex;

use crate::{db_structs::SubCategoria, Database};

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
