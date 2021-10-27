use actix_web::{get, HttpRequest, HttpResponse};
use itconfig::get_env_or_default;
use rand::seq::SliceRandom;
use serde::Deserialize;
use tera::Context;

use crate::assets::TEMPLATES;
use crate::errors::ApiError;
use crate::models::version::Version;

pub mod api;

/// Limiter.top_tier: default (and currently only option): true
#[derive(Debug, Deserialize)]
pub struct Limiter {
  limit: Option<i64>,
  top_tier: Option<bool>,
}

impl Default for Limiter {
  fn default() -> Limiter {
    Limiter {
      limit: Some(10),
      top_tier: Some(true),
    }
  }
}

/// version handler
#[get("/version")]
pub async fn version() -> Result<HttpResponse, ApiError> {
  let app_version: String = String::from(env!("CARGO_PKG_VERSION"));

  Ok(
    HttpResponse::Ok()
      .content_type(mime::APPLICATION_JAVASCRIPT_UTF_8)
      .json(Version {
        version: app_version,
      }),
  )
}

/// calendar handler
#[get("/calendar")]
pub async fn calendar() -> Result<HttpResponse, ApiError> {
  let context = Context::new();
  let resp_body = TEMPLATES.render("calendar.tera.html", &context);

  Ok(
    HttpResponse::Ok()
      .content_type(mime::TEXT_HTML_UTF_8)
      .body(resp_body.unwrap()),
  )
}

/// docs handler
#[get("/docs")]
pub async fn docs() -> Result<HttpResponse, ApiError> {
  let context = Context::new();
  let resp_body = TEMPLATES.render("docs.tera.html", &context);

  Ok(
    HttpResponse::Ok()
      .content_type(mime::TEXT_HTML_UTF_8)
      .body(resp_body.unwrap()),
  )
}

/// news handler
#[get("/news")]
pub async fn news() -> Result<HttpResponse, ApiError> {
  let context = Context::new();
  let resp_body = TEMPLATES.render("news.tera.html", &context);

  Ok(
    HttpResponse::Ok()
      .content_type(mime::TEXT_HTML_UTF_8)
      .body(resp_body.unwrap()),
  )
}

/// index handler
#[get("/")]
pub async fn index(_req: HttpRequest) -> Result<HttpResponse, ApiError> {
  let wisdom_sentences = vec![
    "might get DMCA struck at all times...",
    "get 'em while they're hot...",
    "may go down every second...",
  ];
  let wisdom = wisdom_sentences.choose(&mut rand::thread_rng());
  let app_base_url: String = get_env_or_default("PUBLIC_BASE_URL", "https://hltvapi.f4b.io");

  let mut context = tera::Context::new();
  context.insert(
    "wisdom",
    wisdom.unwrap_or(&"might get DCMA struck at all times..."),
  );
  context.insert("app_base_url", app_base_url.as_str());
  let resp_body = TEMPLATES.render("landing.tera.html", &context);

  Ok(
    HttpResponse::Ok()
      .content_type(mime::TEXT_HTML_UTF_8)
      .body(resp_body.unwrap()),
  )
}

#[cfg(test)]
mod tests {
  use actix_web::http::StatusCode;
  use actix_web::{test, web, App, HttpResponse};

  // use super::index;

  #[actix_rt::test]
  async fn test_index_ok() {
    let app = test::init_service(
      App::new().service(web::resource("/").to(|| async { HttpResponse::Ok() })),
    )
    .await;

    // Create request object
    let req = test::TestRequest::with_uri("/").to_request();

    // Execute application
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    // insta::assert_yaml_snapshot!(resp.status().as_str());
  }
}
