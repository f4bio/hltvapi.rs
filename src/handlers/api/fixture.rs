use std::path::Path;

use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::{get, web, HttpResponse};

use crate::constants::FIXTURES_CALENDAR_FILENAME;
use crate::errors::ApiError;
use crate::handlers::Limiter;
use crate::models::fixture::{Fixture, PublicFixture};

/// get all fixtures
#[get("/fixture/list")]
pub async fn find_all(params: web::Query<Limiter>) -> Result<HttpResponse, ApiError> {
  let inner_params = params.into_inner();
  let fixtures: Vec<PublicFixture> =
    Fixture::filtered_public(inner_params.limit, inner_params.top_tier)?;
  Ok(
    HttpResponse::Ok()
      .content_type(mime::APPLICATION_JSON)
      .json(fixtures),
  )
}

/// get news as ics calendar file
#[get("/fixture/calendar.ics")]
pub async fn calendar_ics() -> Result<NamedFile, ApiError> {
  let path = Path::new("static").join(FIXTURES_CALENDAR_FILENAME);
  let file = NamedFile::open(path).unwrap();
  Ok(
    file
      .set_content_disposition(ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: vec![DispositionParam::Name(String::from(
          FIXTURES_CALENDAR_FILENAME,
        ))],
      })
      .set_content_type("text/calendar".parse().unwrap()),
  )
}

/// extract path info from "/fixture/{hashid}" url
/// {hashid} - deserializes to hashid
#[get("/fixture/{hash}")]
pub async fn find(hash: web::Path<String>) -> Result<HttpResponse, ApiError> {
  let fixtures = Fixture::find_public(hash.into_inner())?;
  Ok(
    HttpResponse::Ok()
      .content_type(mime::APPLICATION_JSON)
      .json(fixtures),
  )
}
