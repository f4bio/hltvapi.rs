use std::path::Path;

use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::{get, web, HttpResponse, Result};

use crate::constants::NEWS_FEED_FILENAME;
use crate::errors::ApiError;
use crate::handlers::Limiter;
use crate::models::news::{News, PublicNews};

/// get all news
#[get("/news/list")]
pub async fn find_all(params: web::Query<Limiter>) -> Result<HttpResponse, ApiError> {
  let inner_params = params.into_inner();
  let news: Vec<PublicNews> = News::filtered_public(inner_params.limit)?;
  Ok(
    HttpResponse::Ok()
      .content_type(mime::APPLICATION_JSON)
      .json(news),
  )
}

/// get news as rss feed xml file
#[get("/news/feed.xml")]
pub async fn feed_xml() -> Result<NamedFile, ApiError> {
  let path = Path::new("static").join(NEWS_FEED_FILENAME);
  let file = NamedFile::open(path).unwrap();
  Ok(
    file
      .set_content_disposition(ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: vec![DispositionParam::Name(String::from(NEWS_FEED_FILENAME))],
      })
      .set_content_type("application/rss+xml".parse().unwrap()),
  )
}

/// extract path info from "/news/{hash}" url
/// {hash} - deserializes to an Uuid
#[get("/news/{hash}")]
pub async fn find(hash: web::Path<String>) -> Result<HttpResponse, ApiError> {
  let news = News::find_public(hash.into_inner())?;
  Ok(
    HttpResponse::Ok()
      .content_type(mime::APPLICATION_JSON)
      .json(news),
  )
}
