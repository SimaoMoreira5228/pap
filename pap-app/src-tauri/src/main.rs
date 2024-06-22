// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db_structs::Livro;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod db_structs;

struct Database {
    pool: Pool,
}

#[derive(Serialize, Deserialize)]
struct LivroAsResponse {
    id: i32,
    nome: String,
    resumo: Option<String>,
    n_paginas: i32,
    idioma: String,
    img_url: Option<String>,
    ano_edicao: Option<String>,
    autor: Option<String>,
    editora: String,
    id_secao: i32,
    categoria: Option<String>,
    sub_categoria: Option<String>,
    requisitado: bool,
}

#[tauri::command]
fn init(db_url: String, state: tauri::State<'_, Mutex<Option<Database>>>) -> Result<(), String> {
    let pool = Pool::new(db_url.as_ref()).map_err(|e| format!("Failed to create pool: {}", e))?;
    *state.lock().unwrap() = Some(Database { pool });
    Ok(())
}

#[tauri::command]
async fn get_books(
    limit: i32,
    offset: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<LivroAsResponse>, String> {
    let state_lock = state.lock().unwrap();
    let db = state_lock.as_ref().ok_or("Database not initialized")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let books = conn
        .exec_map(
            "SELECT * FROM livros LIMIT :limit OFFSET :offset",
            params! { "limit" => limit, "offset" => offset },
            |(
                id,
                nome,
                resumo,
                n_paginas,
                idioma,
                img_url,
                ano_edicao,
                id_autor,
                id_editora,
                id_secao,
                id_sub_categoria,
            )| {
                Livro {
                    id,
                    nome,
                    resumo,
                    n_paginas,
                    idioma,
                    img_url,
                    ano_edicao,
                    id_autor,
                    id_editora,
                    id_secao,
                    id_sub_categoria,
                }
            },
        )
        .map_err(|e| format!("Failed to query: {}", e))?;

    let mut books_as_response = Vec::new();

    for book in books {
        let autor = conn
            .exec_first(
                "SELECT nome FROM autores WHERE id = :id",
                params! { "id" => book.id_autor },
            )
            .map_err(|e| format!("Failed to query: {}", e))?
            .unwrap();

        let categoria = conn.exec_first("SELECT nome FROM categorias WHERE id = (SELECT id_categoria FROM sub_categorias WHERE id = :id)", params! { "id" => book.id_sub_categoria }).map_err(|e| format!("Failed to query: {}", e))?.unwrap();

        let sub_categoria = conn
            .exec_first(
                "SELECT nome FROM sub_categorias WHERE id = :id",
                params! { "id" => book.id_sub_categoria },
            )
            .map_err(|e| format!("Failed to query: {}", e))?
            .unwrap();

        let publisher = conn
            .exec_first(
                "SELECT nome FROM editoras WHERE id = :id",
                params! { "id" => book.id_editora },
            )
            .map_err(|e| format!("Failed to query: {}", e))?
            .unwrap();

        let is_requested = conn
            .exec_first(
                "SELECT COUNT(*) FROM requisicoes WHERE id_livro_requisitado = :id",
                params! { "id" => book.id },
            )
            .map_err(|e| format!("Failed to query: {}", e))?
            .unwrap();

        books_as_response.push(LivroAsResponse {
            id: book.id,
            nome: book.nome,
            resumo: book.resumo,
            n_paginas: book.n_paginas,
            idioma: book.idioma,
            img_url: book.img_url,
            ano_edicao: book.ano_edicao,
            autor,
            editora: publisher,
            id_secao: book.id_secao,
            categoria,
            sub_categoria,
            requisitado: is_requested,
        });
    }

    Ok(books_as_response)
}

#[tauri::command]
async fn get_books_count(state: tauri::State<'_, Mutex<Option<Database>>>) -> Result<i32, String> {
    let state_lock = state.lock().unwrap();
    let db = state_lock.as_ref().ok_or("Database not initialized")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let count = conn
        .exec_first("SELECT COUNT(*) FROM livros", ())
        .map_err(|e| format!("Failed to query: {}", e))?;

    Ok(count.unwrap())
}
fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(None::<Database>))
        .invoke_handler(tauri::generate_handler![init, get_books, get_books_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
