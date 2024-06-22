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
}

#[tauri::command]
fn init(db_url: String, state: tauri::State<'_, Mutex<Option<Database>>>) -> Result<(), String> {
    println!("Initializing database with url: {}", db_url);

    let pool = Pool::new(db_url.as_ref()).map_err(|e| format!("Failed to create pool: {}", e))?;
    *state.lock().unwrap() = Some(Database { pool });
    Ok(())
}

#[tauri::command]
fn get_unrequested_books(
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<LivroAsResponse>, String> {
    let state_lock = state.lock().unwrap();
    let db = state_lock.as_ref().ok_or("Database not initialized")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let books = conn
        .query_map(
            "SELECT * FROM livro WHERE id NOT IN (SELECT id_livro_requisitado FROM requisicao)",
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
                "SELECT nome FROM autor WHERE id = :id",
                params! { "id" => book.id_autor },
            )
            .unwrap()
            .unwrap();

        let categoria = conn.exec_first("SELECT nome FROM categoria WHERE id = (SELECT id_categoria FROM sub_categoria WHERE id = :id)", params! { "id" => book.id_sub_categoria }).unwrap().unwrap();

        let sub_categoria = conn
            .exec_first(
                "SELECT nome FROM sub_categoria WHERE id = :id",
                params! { "id" => book.id_sub_categoria },
            )
            .unwrap()
            .unwrap();

        let publisher = conn
            .exec_first(
                "SELECT nome FROM editora WHERE id = :id",
                params! { "id" => book.id_editora },
            )
            .unwrap()
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
        });
    }

    Ok(books_as_response)
}

#[tauri::command]
fn get_requested_books(
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<LivroAsResponse>, String> {
    let state_lock = state.lock().unwrap();
    let db = state_lock.as_ref().ok_or("Database not initialized")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Failed to get connection: {}", e))?;

    let books = conn
        .query_map(
            "SELECT * FROM livro WHERE id IN (SELECT id_livro_requisitado FROM requisicao)",
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
                "SELECT nome FROM autor WHERE id = :id",
                params! { "id" => book.id_autor },
            )
            .unwrap()
            .unwrap();

        let categoria = conn.exec_first("SELECT nome FROM categoria WHERE id = (SELECT id_categoria FROM sub_categoria WHERE id = :id)", params! { "id" => book.id_sub_categoria }).unwrap().unwrap();

        let sub_categoria = conn
            .exec_first(
                "SELECT nome FROM sub_categoria WHERE id = :id",
                params! { "id" => book.id_sub_categoria },
            )
            .unwrap()
            .unwrap();

        let publisher = conn
            .exec_first(
                "SELECT nome FROM editora WHERE id = :id",
                params! { "id" => book.id_editora },
            )
            .unwrap()
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
        });
    }

    Ok(books_as_response)
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(None::<Database>))
        .invoke_handler(tauri::generate_handler![
            init,
            get_unrequested_books,
            get_requested_books
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
