use tokio::sync::Mutex;

use crate::{
    db_structs::{Bibliotecario, Permissao},
    jwt::{get_from_jwt, new_jwt, verify_jwt},
    Database,
};

#[tauri::command]
pub async fn login(
    name: String,
    password: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<String, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let librarian =
        sqlx::query_as::<_, Bibliotecario>("SELECT * FROM bibliotecarios WHERE nome = ?")
            .bind(name)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar bibliotecário: {}", e);
                format!("Falha ao consultar bibliotecário: {}", e)
            })?;

    if librarian.is_empty() {
        return Err("Nome ou password inválidos".into());
    }

    let is_correct =
        bcrypt::verify(password, &librarian.first().unwrap().password).map_err(|e| {
            tracing::error!("Falha ao verificar password: {}", e);
            format!("Falha ao verificar password: {}", e)
        })?;

    if !is_correct {
        tracing::warn!("Nome ou password inválidos");
        return Err("Nome ou password inválidos".into());
    }

    let librarian = librarian.first().unwrap();

    Ok(new_jwt(&librarian))
}

#[tauri::command]
pub async fn new_librarian(
    token: String,
    name: String,
    password: String,
    role: Option<String>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(), String> {
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

    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    if role.is_some() {
        let role_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM cargos WHERE nome = ?")
            .bind(&role)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar: {}", e);
                format!("Falha ao consultar: {}", e)
            })?;

        if role_count == 0 {
            return Err("Cargo não existe".to_string());
        }

        sqlx::query("INSERT INTO bibliotecarios (nome, password, cargo) VALUES (?, ?, ?)")
            .bind(name)
            .bind(hashed_password)
            .bind(role)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao inserir bibliotecário: {}", e);
                format!("Falha ao inserir bibliotecário: {}", e)
            })?;
    } else {
        sqlx::query("INSERT INTO bibliotecarios (nome, password) VALUES (?, ?)")
            .bind(name)
            .bind(hashed_password)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao inserir bibliotecário: {}", e);
                format!("Falha ao inserir bibliotecário: {}", e)
            })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn check_librarians_existence(
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<bool, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM bibliotecarios")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    Ok(count > 0)
}

#[tauri::command]
pub async fn does_librarian_has_permission(
    token: String,
    permission_id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<bool, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let claims = get_from_jwt(&token).map_err(|e| {
        tracing::error!("Falha ao obter claims: {}", e);
        format!("Falha ao obter claims: {}", e)
    })?;

    let count: i32 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cargos WHERE nome = ? AND permissao = ?")
            .bind(claims.get("role").unwrap())
            .bind(permission_id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar: {}", e);
                format!("Falha ao consultar: {}", e)
            })?;

    Ok(count > 0)
}

#[tauri::command]
pub async fn does_librarian_has_permission_by_acao(
    token: String,
    acao: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<bool, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let claims = get_from_jwt(&token).map_err(|e| {
        tracing::error!("Falha ao obter claims: {}", e);
        format!("Falha ao obter claims: {}", e)
    })?;

    let permission_id: i32 = sqlx::query_scalar("SELECT id FROM permissoes WHERE acao = ?")
        .bind(acao)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    let count: i32 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cargos WHERE nome = ? AND permissao = ?")
            .bind(claims.get("role").unwrap())
            .bind(permission_id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar: {}", e);
                format!("Falha ao consultar: {}", e)
            })?;

    Ok(count > 0)
}

#[tauri::command]
pub async fn get_librarian_permissions(
    token: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Permissao>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let claims = get_from_jwt(&token).map_err(|e| {
        tracing::error!("Falha ao obter claims: {}", e);
        format!("Falha ao obter claims: {}", e)
    })?;

    let permissions: Vec<Permissao> = sqlx::query_as::<_, Permissao>(
        "SELECT permissoes.id, permissoes.acao, permissoes.label FROM permissoes
        WHERE permissoes.id IN (SELECT permissao FROM cargos WHERE nome = ?)",
    )
    .bind(claims.get("role").unwrap())
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Falha ao consultar: {}", e);
        format!("Falha ao consultar: {}", e)
    })?;

    Ok(permissions)
}
