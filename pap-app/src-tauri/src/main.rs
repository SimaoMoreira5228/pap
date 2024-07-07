// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod authors;
mod books;
mod db_structs;
mod jwt;
mod librarians;
mod permissions;
mod publishers;
mod readers;
mod requests;
mod sub_categories;
mod tables;

use authors::{
    create_author, delete_author, get_author_by_id, get_authors, get_authors_by_name,
    get_authors_count, get_books_by_author_id, update_author,
};
use books::{create_book, delete_book, get_book_by_id, get_books, get_books_count, update_book};
use librarians::{
    check_librarians_existence, does_librarian_has_permission,
    does_librarian_has_permission_by_acao, get_librarian_permissions, login, new_librarian,
};
use permissions::{add_permission_to_role, get_permissions};
use publishers::{
    create_publisher, delete_publisher, get_books_by_publisher_id, get_publisher_by_id,
    get_publishers, get_publishers_by_name, get_publishers_count, update_publisher,
};
use readers::{
    create_reader, delete_reader, get_reader_by_id, get_readers, get_readers_by_name,
    get_readers_count, get_requested_books_by_reader_id, update_reader,
};
use requests::{get_requested_book_by_book_id, request_book, return_book};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use sub_categories::get_sub_categories_by_name;
use tables::create_tables;
use tauri::Manager;
use tokio::sync::Mutex;
use tracing_subscriber::FmtSubscriber;

pub struct Database {
    pub pool: Pool<MySql>,
}

lazy_static::lazy_static! {
    static ref SECRET: String = chrono::Utc::now().timestamp().to_string();
}

#[tauri::command]
async fn init(
    db_url: String,
    make_tables: Option<bool>,
    state: tauri::State<'_, Mutex<Option<Database>>>,
) -> Result<(), String> {
    if make_tables.unwrap_or(false) {
        let db_name = db_url.split('/').last().unwrap();
        let no_name_db_url = db_url.replace(db_name, "");

        let no_db_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(no_name_db_url.as_ref())
            .await
            .map_err(|e| {
                tracing::error!("Falha ao criar pool: {}", e);
                format!("Falha ao criar pool: {}", e)
            })?;

        let query = format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name);

        sqlx::query(&query)
            .execute(&no_db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Falha ao criar base de dados: {}", e);
                format!("Falha ao criar base de dados: {}", e)
            })?;

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_ref())
            .await
            .map_err(|e| {
                tracing::error!("Falha ao criar pool: {}", e);
                format!("Falha ao criar pool: {}", e)
            })?;

        *state.lock().await = Some(Database { pool: pool.clone() });

        create_tables(&pool)
            .await
            .map_err(|e| format!("Falha ao criar tabelas: {}", e))?;
    } else {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_ref())
            .await
            .map_err(|e| {
                tracing::error!("Falha ao criar pool: {}", e);
                format!("Falha ao criar pool: {}", e)
            })?;

        *state.lock().await = Some(Database { pool });
    }

    tracing::debug!("Pool criado com sucesso");

    Ok(())
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .manage(Mutex::new(None::<Database>))
        .invoke_handler(tauri::generate_handler![
            // others
            init,
            // books
            get_books,
            get_books_count,
            get_book_by_id,
            create_book,
            update_book,
            delete_book,
            // requests
            request_book,
            get_requested_book_by_book_id,
            return_book,
            // librarians
            login,
            new_librarian,
            check_librarians_existence,
            get_librarian_permissions,
            does_librarian_has_permission_by_acao,
            // permissions
            does_librarian_has_permission,
            get_permissions,
            add_permission_to_role,
            // readers
            get_reader_by_id,
            get_readers_by_name,
            create_reader,
            get_readers,
            get_readers_count,
            update_reader,
            delete_reader,
            get_requested_books_by_reader_id,
            // authors
            get_author_by_id,
            get_authors_by_name,
            get_books_by_author_id,
            get_authors,
            get_authors_count,
            create_author,
            update_author,
            delete_author,
            // publishers
            get_publisher_by_id,
            get_publishers_by_name,
            get_books_by_publisher_id,
            get_publishers,
            get_publishers_count,
            create_publisher,
            update_publisher,
            delete_publisher,
            // sub-categories
            get_sub_categories_by_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
