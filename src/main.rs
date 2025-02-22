use actix_web::{cookie::{Cookie, SameSite}, get, post, web, App, HttpResponse, HttpServer, Result, HttpRequest};

#[post("/{cookie_name}")]
async fn set_cookie(
    path: web::Path<String>,
    request_body: String,
) -> Result<HttpResponse> {
    let cookie_name = path.into_inner();
    let cookie_value = request_body;

    let cookie = Cookie::build(cookie_name, cookie_value)
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .finish())
}

#[get("/{cookie_name}")]
async fn replay_cookie(
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let cookie_name = path.into_inner();
    
    match req.cookie(&cookie_name) {
        Some(cookie) => Ok(HttpResponse::Ok().body(cookie.value().to_string())),
        #[allow(non_snake_case)]
        None => Ok(HttpResponse::NotFound().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(set_cookie)
        .service(replay_cookie))
        .bind(("0.0.0.0", 1317))? // Bind to all interfaces
        .run()
        .await
}