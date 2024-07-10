use tokio::sync::Mutex;

use crate::{
    db_structs::{Categoria, SubCategoria},
    jwt::verify_jwt,
    Database,
};

#[tauri::command]
pub async fn get_categories(
    token: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Categoria>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let pool = &db.pool;

    verify_jwt(&token, &pool).await.map_err(|e| {
        tracing::error!("Falha ao verificar token: {}", e);
        format!("Falha ao verificar token: {}", e)
    })?;

    let sub_categories = sqlx::query_as::<_, Categoria>("SELECT * FROM categorias")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar categorias: {}", e);
            format!("Falha ao consultar categorias: {}", e)
        })?;

    Ok(sub_categories)
}

#[tauri::command]
pub async fn delete_category(
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

    let sub_categories =
        sqlx::query_as::<_, SubCategoria>("SELECT * FROM sub_categorias WHERE id_categoria = ?")
            .bind(id)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar subcategorias: {}", e);
                format!("Falha ao consultar subcategorias: {}", e)
            })?;

    for sub_category in sub_categories {
        let used = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT * FROM livros WHERE id_sub_categoria = ?)",
        )
        .bind(sub_category.id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar se sub-categoria é usada: {}", e);
            format!("Falha ao consultar se sub-categoria é usada: {}", e)
        })?;

        if used {
            return Err(
                "Esta categoria contem uma sub-categoria que tem livros associados, não é possível apagá-la".to_string(),
            );
        }
    }

    sqlx::query("DELETE FROM categorias WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao deletar categoria: {}", e);
            format!("Falha ao deletar categoria: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn create_category(
    token: String,
    name: String,
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

    let exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT * FROM categorias WHERE nome = ?)")
            .bind(&name)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar se categoria já existe: {}", e);
                format!("Falha ao consultar se categoria já existe: {}", e)
            })?;

    if exists {
        return Err("Esta categoria já existe".to_string());
    }

    sqlx::query("INSERT INTO categorias (nome) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao inserir categoria: {}", e);
            format!("Falha ao inserir categoria: {}", e)
        })?;

    Ok(())
}
