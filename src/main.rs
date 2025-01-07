use actix_web::{web, App, HttpServer};
    use dotenv::dotenv;
    use std::env;

    mod handlers;
    mod utils;

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
      dotenv().ok();
      let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
      println!("Servidor rodando em http://0.0.0.0:{}", port);
      HttpServer::new(|| {
        App::new()
          .service(handlers::check_downloadable)
      })
      .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
      .run()
      .await
    }
