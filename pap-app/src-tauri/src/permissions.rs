use tokio::sync::Mutex;

use crate::{db_structs::Permissao, jwt::verify_jwt, Database};

#[tauri::command]
pub async fn add_permission_to_role(
    token: String,
    role: String,
    permissions: Vec<i32>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<String, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    if token == "first_librarian" {
        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM bibliotecarios")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar: {}", e);
                format!("Falha ao consultar: {}", e)
            })?;

        if count > 0 {
            return Err("Já existe um bibliotecário".to_string());
        }
    } else {
        verify_jwt(&token, &pool).await.map_err(|e| {
            tracing::error!("Falha ao verificar token: {}", e);
            format!("Falha ao verificar token: {}", e)
        })?;
    }

    for permission in permissions {
        let role_count: i32 =
            sqlx::query_scalar("SELECT COUNT(*) FROM cargos WHERE nome = ? AND permissao = ?")
                .bind(&role)
                .bind(permission)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Falha ao consultar: {}", e);
                    format!("Falha ao consultar: {}", e)
                })?;

        if role_count > 0 {
            return Err("Cargo já tem essa permissão".to_string());
        }

        sqlx::query("INSERT INTO cargos (nome, permissao) VALUES (?, ?)")
            .bind(&role)
            .bind(permission)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao inserir permissão: {}", e);
                format!("Falha ao inserir permissão: {}", e)
            })?;
    }

    Ok(role)
}

#[tauri::command]
pub async fn get_permissions(
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Permissao>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let permissions = sqlx::query_as::<_, Permissao>("SELECT * FROM permissoes")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    Ok(permissions)
}
