use actix_web::{web, App, HttpResponse, HttpServer, Responder, cookie::Cookie};
mod secrets;

pub const COOKIE_NAME: &str = "auth_cookie";

async fn get_keys(req: actix_web::HttpRequest) -> impl Responder {
    // Check for the cookie
    if let Some(cookie) = req.cookie(COOKIE_NAME) {
        if cookie.value() == secrets::REQUIRED_COOKIE_VALUE {
            HttpResponse::Ok().json(secrets::keys())
        } else {
            HttpResponse::Unauthorized().finish()
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

async fn set_auth_cookie(body: String) -> impl Responder {
    // let cookie = Cookie::new(COOKIE_NAME, body);
    let cookie = Cookie::build(COOKIE_NAME, body)
    .secure(true)
    .http_only(true)
    .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/set_auth_cookie", web::post().to(set_auth_cookie))
            .route("/", web::get().to(get_keys))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}