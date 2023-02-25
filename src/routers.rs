use actix_web::{get, post, HttpResponse, Responder, web::Json, error::Error};
use simple_sns::models::{PostUser, SignInUser, JwtData};
use simple_sns::cruds::{create_user, check_user};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;
use jwt::SignWithKey;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/sign_up")]
pub async fn registration_user(post_user: Json<PostUser>) -> Result<HttpResponse, Error> {
    let new_user = create_user(&post_user.email, &post_user.name, &post_user.password);
    Ok(HttpResponse::Ok().json(new_user))
}

#[post("/sign_in")]
pub async fn get_jwt(signin_user: Json<SignInUser>) -> Result<HttpResponse, Error> {
    let result = check_user(&signin_user.email, &signin_user.password);
    let value = match result {
        Ok(value) => value,
        Err(e) =>{
            return Ok(HttpResponse::BadRequest().body(e.to_string()))
        }
    };
    // ここでvalue(ユーザ情報)の一部からjwtを作り、JwtDataに入れる。
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", value.id);
    let jwt = claims.sign_with_key(&key).unwrap();
    let jwt_data = JwtData { jwt };
    Ok(HttpResponse::Ok().json(jwt_data))
}
