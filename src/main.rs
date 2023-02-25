use actix_web::{App, HttpServer};
mod routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routers::hello)
            .service(routers::registration_user)
            .service(routers::get_jwt)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
