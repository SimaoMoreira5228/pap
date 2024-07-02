// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db_structs::{Bibliotecario, Livro, Permissao};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio::sync::Mutex;

mod db_structs;

struct Database {
    pool: Pool,
}

lazy_static::lazy_static! {
    static ref SECRET: String = chrono::Utc::now().timestamp().to_string();
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

fn verify_jwt(token: &str, conn: &mut PooledConn) -> Result<(), String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes()).unwrap();
    let claims: Result<std::collections::BTreeMap<String, String>, _> = token.verify_with_key(&key);

    if claims.is_err() {
        return Err("token inválido".to_string());
    }

    let claims = claims.unwrap();

    let id = claims.get("id").unwrap().parse::<i32>().unwrap();

    let librarian = conn.exec_map(
        "SELECT * FROM bibliotecarios WHERE id = :id LIMIT 1",
        params! { "id" => id },
        |(id, nome, password, cargo)| Bibliotecario {
            id,
            nome,
            password,
            cargo,
        },
    );

    if librarian.is_err() {
        return Err("Nenhum bibliotecário encontrado".to_string());
    }

    Ok(())
}

#[tauri::command]
async fn init(
    db_url: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(), String> {
    let pool = Pool::new(db_url.as_ref()).map_err(|e| format!("Falha ao criar pool: {}", e))?;
    *state.lock().await = Some(Database { pool });
    Ok(())
}

#[tauri::command]
async fn get_books(
    token: String,
    limit: i32,
    offset: i32,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<LivroAsResponse>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    verify_jwt(&token, &mut conn).map_err(|e| format!("Falha ao verificar token: {}", e))?;

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
        .map_err(|e| format!("Falha ao consultar: {}", e))?;

    let mut books_as_response = Vec::new();

    for book in books {
        let autor = conn
            .exec_first(
                "SELECT nome FROM autores WHERE id = :id",
                params! { "id" => book.id_autor },
            )
            .map_err(|e| format!("Falha ao consultar: {}", e))?
            .unwrap();

        let categoria = conn.exec_first("SELECT nome FROM categorias WHERE id = (SELECT id_categoria FROM sub_categorias WHERE id = :id)", params! { "id" => book.id_sub_categoria }).map_err(|e| format!("Falha ao consultar: {}", e))?.unwrap();

        let sub_categoria = conn
            .exec_first(
                "SELECT nome FROM sub_categorias WHERE id = :id",
                params! { "id" => book.id_sub_categoria },
            )
            .map_err(|e| format!("Falha ao consultar: {}", e))?
            .unwrap();

        let publisher = conn
            .exec_first(
                "SELECT nome FROM editoras WHERE id = :id",
                params! { "id" => book.id_editora },
            )
            .map_err(|e| format!("Falha ao consultar: {}", e))?
            .unwrap();

        let is_requested = conn
            .exec_first(
                "SELECT COUNT(*) FROM requisicoes WHERE id_livro_requisitado = :id",
                params! { "id" => book.id },
            )
            .map_err(|e| format!("Falha ao consultar: {}", e))?
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
async fn get_books_count(
    token: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<i32, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    verify_jwt(&token, &mut conn).map_err(|e| format!("Falha ao verificar token: {}", e))?;

    let count = conn
        .exec_first("SELECT COUNT(*) FROM livros", ())
        .map_err(|e| format!("Falha ao consultar: {}", e))?;

    Ok(count.unwrap())
}

#[tauri::command]
async fn login(
    name: String,
    password: String,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<String, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    let librarian = conn
        .exec_map(
            "SELECT * FROM bibliotecarios WHERE nome = :name LIMIT 1",
            params! { "name" => &name },
            |(id, nome, password, cargo)| Bibliotecario {
                id,
                nome,
                password,
                cargo,
            },
        )
        .map_err(|e| format!("Falha ao consultar: {}", e))
        .unwrap();

    if librarian.is_empty() {
        return Err("Nome ou password inválidos".into());
    }

    let is_correct = bcrypt::verify(password, &librarian.first().unwrap().password)
        .map_err(|e| format!("Falha ao verificar senha: {}", e))?;

    if !is_correct {
        return Err("Nome ou password inválidos".into());
    }

    let librarian = librarian.first().unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes()).unwrap();
    let mut claims = std::collections::BTreeMap::new();

    let id_as_string = &librarian.id.to_string();

    claims.insert("id", id_as_string);
    claims.insert("name", &librarian.nome);
    claims.insert("role", &librarian.cargo);

    let token_str = claims.sign_with_key(&key).unwrap();

    Ok(token_str)
}

#[tauri::command]
async fn new_librarian(
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

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    if token == "first_librarian" {
        let count: Option<i32> = conn
            .exec_first("SELECT COUNT(*) FROM bibliotecarios", ())
            .map_err(|e| format!("Falha ao consultar: {}", e))?;

        if count.unwrap() > 0 {
            return Err("Já existe um bibliotecário".to_string());
        }
    } else {
        verify_jwt(&token, &mut conn).map_err(|e| format!("Falha ao verificar token: {}", e))?;
    }

    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    if role.is_some() {
        let role_count: Option<i32> = conn
            .exec_first(
                "SELECT COUNT(*) FROM cargos WHERE nome = :role",
                params! { "role" => &role },
            )
            .map_err(|e| format!("Falha ao consultar: {}", e))?;

        if role_count.unwrap() == 0 {
            return Err("Cargo não existe".to_string());
        }

        conn.exec_drop(
            "INSERT INTO bibliotecarios (nome, password, cargo) VALUES (:name, :password, :role)",
            params! { "name" => name, "password" => hashed_password, "role" => role },
        )
        .map_err(|e| format!("Failed to insert librarian: {}", e))?;
    } else {
        conn.exec_drop(
            "INSERT INTO bibliotecarios (nome, password) VALUES (:name, :password)",
            params! { "name" => name, "password" => hashed_password },
        )
        .map_err(|e| format!("Failed to insert librarian: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
async fn add_permission_to_role(
    token: String,
    role: String,
    permissions: Vec<i32>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<String, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    if token == "first_librarian" {
        let count: Option<i32> = conn
            .exec_first("SELECT COUNT(*) FROM bibliotecarios", ())
            .map_err(|e| format!("Falha ao consultar: {}", e))?;

        if count.unwrap() > 0 {
            return Err("Já existe um bibliotecário".to_string());
        }
    } else {
        verify_jwt(&token, &mut conn).map_err(|e| format!("Falha ao verificar token: {}", e))?;
    }

    for permission in permissions {
        let role_count: Option<i32> = conn
            .exec_first(
                "SELECT COUNT(*) FROM cargos WHERE nome = :role AND permissao = :permission",
                params! { "role" => &role, "permission" => permission },
            )
            .map_err(|e| format!("Falha ao consultar: {}", e))?;

        if role_count.unwrap() > 0 {
            return Err("Cargo já tem essa permissão".to_string());
        }

        conn.exec_drop(
            "INSERT INTO cargos (nome, permissao) VALUES (:role, :permission)",
            params! { "role" => &role, "permission" => permission },
        )
        .map_err(|e| format!("Falha ao inserir permissão: {}", e))?;
    }

    Ok(role)
}

#[tauri::command]
async fn check_librarians_existence(
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<bool, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    let count: Option<i32> = conn
        .exec_first("SELECT COUNT(*) FROM bibliotecarios", ())
        .map_err(|e| format!("Falha ao consultar: {}", e))?;

    Ok(count.unwrap() > 0)
}

#[tauri::command]
async fn get_permissions(
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<Vec<Permissao>, String> {
    let state_lock = state.lock().await;
    let db = state_lock
        .as_ref()
        .ok_or("Base de dados não inicializada")?;

    let mut conn = db
        .pool
        .get_conn()
        .map_err(|e| format!("Falha ao conseguir conexão: {}", e))?;

    let permissions: Vec<Permissao> = conn
        .exec_map("SELECT * FROM permissoes", (), |(id, acao, label)| {
            Permissao { id, acao, label }
        })
        .map_err(|e| format!("Falha ao consultar: {}", e))
        .unwrap();

    Ok(permissions)
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(None::<Database>))
        .invoke_handler(tauri::generate_handler![
            init,
            get_books,
            get_books_count,
            login,
            new_librarian,
            check_librarians_existence,
            get_permissions,
            add_permission_to_role
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
