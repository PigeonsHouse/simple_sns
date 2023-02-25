use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;
use chrono::NaiveDateTime;

// sign_upペイロードBody
#[derive(Deserialize)]
pub struct PostUser {
    pub email: String,
    pub name: String,
    pub password: String
}

// sign_inペイロードBody
#[derive(Deserialize)]
pub struct SignInUser {
    pub email: String,
    pub password: String
}

// DBのinsert用Struct
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a String,
    pub name: &'a String,
    pub password_hash: &'a String
}

// DBのselect用struct
#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Serialize)]
pub struct UserPassword {
    pub id: Uuid,
    pub password_hash: String
}

// jwtのレスポンスモデル
#[derive(Serialize)]
pub struct JwtData {
    pub jwt: String
}