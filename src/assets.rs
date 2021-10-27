use actix_web::{get, web, HttpResponse};
use include_dir::Dir;
use lazy_static::lazy_static;
use tera::Tera;
use tracing::{debug, error};

use crate::errors::ApiError;

pub static RESOURCES: Dir<'static> = include_dir!("./web/dist/");

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let mut tera = Tera::default();
    // let template_names = vec![
    //   "base.tera.html",
    //   "calendar.tera.html",
    //   "docs.tera.html",
    //   "landing.tera.html",
    //   "news.tera.html",
    //   "snackbar.tera.html",
    // ];
    tera.add_raw_templates(vec![
      (
        "base.tera.html",
        RESOURCES
          .get_file("base.tera.html")
          .unwrap()
          .contents_utf8()
          .unwrap(),
      ),
      (
        "calendar.tera.html",
        RESOURCES
          .get_file("calendar.tera.html")
          .unwrap()
          .contents_utf8()
          .unwrap(),
      ),
      (
        "docs.tera.html",
        RESOURCES
          .get_file("docs.tera.html")
          .unwrap()
          .contents_utf8()
          .unwrap(),
      ),
      (
        "landing.tera.html",
        RESOURCES
          .get_file("landing.tera.html")
          .unwrap()
          .contents_utf8()
          .unwrap(),
      ),
      (
        "news.tera.html",
        RESOURCES
          .get_file("news.tera.html")
          .unwrap()
          .contents_utf8()
          .unwrap(),
      ),
      (
        "snackbar.tera.html",
        RESOURCES
          .get_file("snackbar.tera.html")
          .unwrap()
          .contents_utf8()
          .unwrap(),
      ),
    ])
    .unwrap();
  tera.autoescape_on(vec!["html"]);
  // tera.register_filter("do_nothing", do_nothing_filter);
  tera
};
}

/// resource handler
#[get("/{resource}")]
pub async fn resources(resource: web::Path<String>) -> Result<HttpResponse, ApiError> {
  let response = if !resource.clone().is_ascii() {
    HttpResponse::Forbidden().body(format!(
      "invalid file name: '{}'",
      resource.clone().as_str()
    ))
  } else if !RESOURCES.contains(resource.clone().as_str()) {
    error!(
      "requested resource '{}' not found!",
      resource.clone().as_str()
    );
    HttpResponse::NotFound().body(format!("no such file: '{}'", resource.clone().as_str()))
  } else {
    // maybe unsafe?
    // -> template (*.tera.html) could be requested directly
    // TODO: double check this
    let resource_file = RESOURCES.get_file(resource.clone().as_str());
    let file_mime_guess = mime_guess::from_path(resource.clone().as_str());

    debug!(
      "file: {} mime: {}",
      resource_file.clone().unwrap().path,
      file_mime_guess.first().unwrap()
    );

    HttpResponse::Ok()
      .content_type(file_mime_guess.first().unwrap())
      .body(resource_file.unwrap().contents())
  };

  Ok(response)
}
