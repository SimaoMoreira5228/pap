use tokio::sync::Mutex;

use crate::{
    db_structs::{Livro, LivroAsResponse},
    jwt::verify_jwt,
    Database,
};

#[tauri::command]
pub async fn get_books(
    token: String,
    limit: i32,
    offset: i32,
    search: Option<String>,
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

    let books;

    if search.is_none() {
        books = sqlx::query_as::<_, Livro>("SELECT * FROM livros LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar livros: {}", e);
                format!("Falha ao consultar livros: {}", e)
            })?;
    } else {
        books = sqlx::query_as::<_, Livro>(
            "SELECT * FROM livros WHERE LOWER(nome) LIKE LOWER(?) LIMIT ? OFFSET ?",
        )
        .bind(format!("%{}%", search.unwrap().to_lowercase()))
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar livros: {}", e);
            format!("Falha ao consultar livros: {}", e)
        })?;
    }

    let mut books_as_response = Vec::new();

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

        let is_requested: i32 =
            sqlx::query_scalar("SELECT COUNT(*) FROM requisicoes WHERE id_livro_requisitado = ? AND data_entrega IS NULL")
                .bind(book.id)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Falha ao consultar requisição: {}", e);
                    format!("Falha ao consultar requisição: {}", e)
                })?;

        books_as_response.push(LivroAsResponse {
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

    Ok(books_as_response)
}

#[tauri::command]
pub async fn get_book_by_id(
    token: String,
    id: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<LivroAsResponse, String> {
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
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar livro: {}", e);
            format!("Falha ao consultar livro: {}", e)
        })?;

    if book.is_empty() {
        tracing::error!("Livro não encontrado");
        return Err("Livro não encontrado".to_string());
    }

    let book = book.first().unwrap().clone();

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

    Ok(LivroAsResponse {
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
    })
}

#[tauri::command]
pub async fn get_books_count(
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
        count = sqlx::query_scalar("SELECT COUNT(*) FROM livros")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar livros: {}", e);
                format!("Falha ao consultar livros: {}", e)
            })?;
    } else {
        count = sqlx::query_scalar("SELECT COUNT(*) FROM livros WHERE LOWER(nome) LIKE LOWER(?)")
            .bind(format!("%{}%", search.unwrap().to_lowercase()))
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao consultar livros: {}", e);
                format!("Falha ao consultar livros: {}", e)
            })?;
    }

    Ok(count)
}

#[tauri::command]
pub async fn create_book(
    token: String,
    name: String,
    resume: Option<String>,
    n_pages: String,
    language: String,
    img_url: Option<String>,
    ano_edicao: Option<String>,
    author_id: String,
    publisher_id: String,
    sub_category_id: String,
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

    sqlx::query("INSERT INTO livros (nome, resumo, n_paginas, idioma, img_url, ano_edicao, id_autor, id_editora, id_sub_categoria) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(name)
        .bind(resume)
        .bind(n_pages)
        .bind(language)
        .bind(img_url)
        .bind(ano_edicao)
        .bind(author_id)
        .bind(publisher_id)
        .bind(sub_category_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao criar livro: {}", e);
            format!("Falha ao criar livro: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn update_book(
    token: String,
    id: i32,
    name: String,
    resume: Option<String>,
    n_pages: String,
    language: String,
    img_url: Option<String>,
    ano_edicao: Option<String>,
    author_id: String,
    publisher_id: String,
    sub_category_id: String,
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

    sqlx::query("UPDATE livros SET nome = ?, resumo = ?, n_paginas = ?, idioma = ?, img_url = ?, ano_edicao = ?, id_autor = ?, id_editora = ?, id_sub_categoria = ? WHERE id = ?")
        .bind(name)
        .bind(resume)
        .bind(n_pages)
        .bind(language)
        .bind(img_url)
        .bind(ano_edicao)
        .bind(author_id)
        .bind(publisher_id)
        .bind(sub_category_id)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao atualizar livro: {}", e);
            format!("Falha ao atualizar livro: {}", e)
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_book(
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

    sqlx::query("DELETE FROM livros WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao deletar livro: {}", e);
            format!("Falha ao deletar livro: {}", e)
        })?;

    Ok(())
}
