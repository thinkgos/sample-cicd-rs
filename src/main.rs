use std::net::TcpListener;

use actix_web::{web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 获取 http server 地址
    let address = format!("{}:{}", "0.0.0.0", "6666");
    // 创建监听器
    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || App::new().route("/healthy", web::get().to(health_check)))
        .listen(listener)?
        .run()
        .await
}
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
