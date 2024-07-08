use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{db_structs::Cargo, jwt::verify_jwt, Database};

#[tauri::command]
pub async fn get_roles(
    token: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Cargo>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let roles = sqlx::query_as::<_, Cargo>("SELECT nome, permissao FROM cargos")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar cargos: {}", e);
            format!("Falha ao consultar cargos: {}", e)
        })?;

    Ok(roles)
}

#[tauri::command]
pub async fn create_role(
    token: String,
    role: String,
    permissions: Vec<i32>,
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

    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM cargos WHERE nome = ?")
        .bind(&role)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if count > 0 {
        return Err("Cargo já existe".to_string());
    }

    for permission in permissions {
        let count = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM cargos WHERE nome = ? AND permissao = ?",
        )
        .bind(&role)
        .bind(&permission)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

        if count > 0 {
            continue;
        }

        sqlx::query("INSERT INTO cargos (nome, permissao) VALUES (?, ?)")
            .bind(&role)
            .bind(permission)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao inserir cargo: {}", e);
                format!("Falha ao inserir cargo: {}", e)
            })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_role(
    token: String,
    role: String,
    new_role_name: Option<String>,
    permissions: Vec<i32>,
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

    sqlx::query("DELETE FROM cargos WHERE nome = ?")
        .bind(&role)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao excluir cargo: {}", e);
            format!("Falha ao excluir cargo: {}", e)
        })?;

    for permission in permissions {
        let count = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM cargos WHERE nome = ? AND permissao = ?",
        )
        .bind(&role)
        .bind(&permission)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

        if count > 0 {
            return Err("Cargo já possui permissão".to_string());
        }

        sqlx::query("INSERT INTO cargos (nome, permissao) VALUES (?, ?)")
            .bind(&role)
            .bind(permission)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao inserir cargo: {}", e);
                format!("Falha ao inserir cargo: {}", e)
            })?;
    }

    if new_role_name.is_some() {
        sqlx::query("UPDATE cargos SET nome = ? WHERE nome = ?")
            .bind(&role)
            .bind(&new_role_name.unwrap())
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao atualizar cargo: {}", e);
                format!("Falha ao atualizar cargo: {}", e)
            })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_role(
    token: String,
    role: String,
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

    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM cargos WHERE nome = ?")
        .bind(&role)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if count == 0 {
        return Err("Cargo não existe".to_string());
    }

    sqlx::query("DELETE FROM cargos WHERE nome = ?")
        .bind(&role)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao excluir cargo: {}", e);
            format!("Falha ao excluir cargo: {}", e)
        })?;

    sqlx::query("UPDATE bibliotecarios SET cargo = NULL WHERE cargo = ?")
        .bind(&role)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao atualizar bibliotecário: {}", e);
            format!("Falha ao atualizar bibliotecário: {}", e)
        })?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct RoleResponse {
    pub name: String,
    pub permissions: Vec<i32>,
}

#[tauri::command]
pub async fn get_role_by_name(
    token: String,
    role: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<RoleResponse, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let roles = sqlx::query_as::<_, Cargo>("SELECT nome, permissao FROM cargos WHERE nome = ?")
        .bind(&role)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar cargo: {}", e);
            format!("Falha ao consultar cargo: {}", e)
        })?;

    let mut role: RoleResponse = RoleResponse {
        name: roles[0].nome.clone(),
        permissions: vec![],
    };

    for r in roles {
        role.permissions.push(r.permissao);
    }

    Ok(role)
}
