use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use sqlx::{MySql, Pool};

use crate::{db_structs::Bibliotecario, SECRET};

pub async fn verify_jwt(token: &str, conn: &Pool<MySql>) -> Result<(), String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes()).unwrap();
    let claims: Result<std::collections::BTreeMap<String, String>, _> = token.verify_with_key(&key);

    if claims.is_err() {
        return Err("token inválido".to_string());
    }

    let claims = claims.unwrap();

    let id = claims.get("id").unwrap().parse::<i32>().unwrap();

    let librarian = sqlx::query_as::<_, Bibliotecario>("SELECT * FROM bibliotecarios WHERE id = ?")
        .bind(id)
        .fetch_all(conn)
        .await
        .map_err(|e| {
            tracing::error!("Falha ao consultar: {}", e);
            format!("Falha ao consultar: {}", e)
        })?;

    if librarian.is_empty() {
        return Err("Nenhum bibliotecário encontrado".to_string());
    }

    Ok(())
}

pub fn new_jwt(librarian: &Bibliotecario) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes()).unwrap();
    let mut claims = std::collections::BTreeMap::new();

    let id_as_string = &librarian.id.to_string();

    claims.insert("id", id_as_string);
    claims.insert("name", &librarian.nome);
    claims.insert("role", &librarian.cargo);

    claims.sign_with_key(&key).unwrap()
}

pub fn get_from_jwt(token: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes()).unwrap();
    let claims: Result<std::collections::BTreeMap<String, String>, _> = token.verify_with_key(&key);

    if claims.is_err() {
        return Err("token inválido".to_string());
    }

    Ok(claims.unwrap())
}
