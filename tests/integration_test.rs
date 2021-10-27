#[cfg(test)]
mod tests {
  use actix_web::{middleware, App, HttpServer};

  //noinspection DuplicatedCode
  #[actix_rt::test]
  async fn test_startup_ok() {
    HttpServer::new(move || {
      App::new()
        // enable logger - always register actix-web Logger middleware last
        .wrap(middleware::Logger::default())
    })
    .bind("localhost:8000".to_string())
    .unwrap()
    .run();
  }
}
