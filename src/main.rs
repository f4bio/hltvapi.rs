#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate include_dir;

use std::time::Duration;
use std::{fs, io};

use actix_web::{web, App, HttpServer};
use clokwerk::{AsyncScheduler, Interval, Job, TimeUnits};
use itconfig::get_env_or_default;
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;

mod assets;
mod constants;
mod database;
mod errors;
mod handlers;
mod logging;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv::dotenv().ok();
  debug!("environment variables initialized!");

  let app_log: String = get_env_or_default("APP_LOG_LEVEL", "warn");
  let subscriber = logging::get_subscriber("hltvapi".into(), app_log);
  logging::init_subscriber(subscriber);
  debug!("core-logging initialized!");

  // create db with connection pool
  database::initialize().ok();
  debug!("database initialized!");

  // lazy_static::initialize(&WEB_TEMPLATES);
  // debug!("templates initialized!");

  debug!("bundled web resources:");
  for entry in assets::RESOURCES.files() {
    debug!("\t* {}", entry.path().display());
  }

  // setting up file storage
  fs::create_dir(constants::FILE_STORAGE_LOCATION).unwrap_or_default();
  debug!("file storage created!");

  openssl_probe::init_ssl_cert_env_vars();
  debug!("openssl probed!");

  info!("setting up server...");

  let server_host: String = get_env_or_default("APP_SERVER_HOST", "0.0.0.0");
  let server_port: usize = get_env_or_default("APP_SERVER_PORT", 1337);
  let server_workers: usize = get_env_or_default("APP_SERVER_WORKERS", 8);

  let server_address = format!("{}:{}", server_host, server_port);
  debug!("server will be listening on 'http://{}'...", server_address);

  // services::fixtures::scrape2().await;
  services::fixtures::scrape_fixtures().await;
  services::news::scrape_news().await;

  let scraper_timeout: u32 = get_env_or_default("APP_SCRAPER_TIMEOUT", 10);
  let scheduler_timeout: Interval = Interval::Minutes(scraper_timeout);
  let mut async_scheduler = AsyncScheduler::new();

  async_scheduler
    .every(scheduler_timeout)
    .plus(5.seconds())
    .run(|| async { info!("=❤=❤= heartbeat =❤=❤=") });
  async_scheduler
    .every(scheduler_timeout)
    .plus(10.seconds())
    .run(services::fixtures::scrape_fixtures);
  async_scheduler
    .every(scheduler_timeout)
    .plus(20.seconds())
    .run(services::news::scrape_news);

  // Spawn a task to run it forever
  tokio::spawn(async move {
    loop {
      async_scheduler.run_pending().await;
      tokio::time::sleep(Duration::from_secs((scraper_timeout * 60 / 2) as u64)).await;
    }
  });

  HttpServer::new(move || {
    App::new()
      // .wrap(middleware::Logger::default())
      .wrap(TracingLogger::default())
      .service(
        web::scope("/api")
          .service(handlers::api::fixture::calendar_ics)
          .service(handlers::api::fixture::find_all)
          .service(handlers::api::fixture::find)
          .service(handlers::api::news::feed_xml)
          .service(handlers::api::news::find_all)
          .service(handlers::api::news::find),
      )
      .service(handlers::calendar)
      .service(handlers::docs)
      .service(handlers::news)
      .service(assets::resources)
      .service(handlers::index)
  })
  .bind(&server_address)
  .unwrap()
  .workers(server_workers)
  .run()
  .await
}

#[cfg(test)]
mod tests {
  use serde::{Deserialize, Serialize};

  #[derive(Clone, Serialize, Deserialize)]
  struct TestObj {
    result: String,
    number: i32,
  }

  //noinspection DuplicatedCode
  #[actix_rt::test]
  async fn test_startup_ok() {
    // TODO: do with server in thread and real http client call - if this is even possible
    tokio::spawn(async move {
      // let resp_body = TestObj {
      //   result: "Ok".to_string(),
      //   number: 0,
      // };
      // HttpServer::new(move || App::new().wrap(middleware::Logger::default()))
      //   .bind("localhost:8000".to_string())
      //   .unwrap()
      //   .run()
      //   .await;
    });

    let resp = TestObj {
      result: "dummy".to_string(),
      number: -1337,
    };
    // insta::assert_yaml_snapshot!(resp);
    assert_eq!(resp.number, -1337)
  }
}
