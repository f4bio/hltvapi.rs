use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database;
use crate::errors::ApiError;
use crate::schema::news;

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Debug, Clone)]
#[table_name = "news"]
pub struct News {
  pub id: i32,
  pub title: String,
  pub link: String,
  pub pub_date: NaiveDateTime,
  pub image: Option<String>,
  pub image_text: Option<String>,
  pub content: Option<String>,
  pub description: Option<String>,
  pub author: Option<String>,
  pub hash: String,
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "news"]
pub struct NewNews {
  pub title: String,
  pub link: String,
  pub pub_date: NaiveDateTime,
  pub image: Option<String>,
  pub image_text: Option<String>,
  pub content: Option<String>,
  pub description: Option<String>,
  pub author: Option<String>,
  pub hash: String,
}

/// use new to send object to data store (sqlite)
#[derive(Deserialize, Serialize)]
pub struct PublicNews {
  pub title: String,
  pub link: String,
  pub pub_date: NaiveDateTime,
  pub image: Option<String>,
  pub image_text: Option<String>,
  pub content: Option<String>,
  pub description: Option<String>,
  pub author: Option<String>,
  pub hash: String,
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

/// use dto to send data between modules
#[derive(Deserialize, Serialize)]
pub struct NewsDTO {
  pub id: Option<i32>,
  pub title: Option<String>,
  pub link: Option<String>,
  pub pub_date: Option<NaiveDateTime>,
  pub image: Option<String>,
  pub image_text: Option<String>,
  pub content: Option<String>,
  pub description: Option<String>,
  pub author: Option<String>,
  pub hash: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
}

impl News {
  pub fn all() -> Result<Vec<Self>, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let res = news.load::<News>(&conn)?;
    Ok(res)
  }

  pub fn all_public() -> Result<Vec<PublicNews>, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let db_result: Vec<News> = news.load::<News>(&conn)?;
    let res: Vec<PublicNews> = db_result.into_iter().map(|n| PublicNews::from(n)).collect();
    Ok(res)
  }

  pub fn filtered_public(limit_to: Option<i64>) -> Result<Vec<PublicNews>, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let limit = limit_to.unwrap_or(10);
    let db_result: Vec<News> = news.limit(limit).load::<News>(&conn)?;
    let res: Vec<PublicNews> = db_result.into_iter().map(PublicNews::from).collect();
    Ok(res)
  }

  pub fn get(id_to_get: i32) -> Result<Self, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let res = news.filter(id.eq(id_to_get)).first(&conn)?;
    Ok(res)
  }

  pub fn find(hash_to_find: String) -> Result<Self, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let res = news.filter(hash.eq(hash_to_find)).first(&conn)?;
    Ok(res)
  }

  pub fn find_public(hash_to_find: String) -> Result<PublicNews, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let db_result: News = news.filter(hash.eq(hash_to_find)).first(&conn)?;
    let res: PublicNews = PublicNews::from(db_result);
    Ok(res)
  }

  pub fn create(new_news: NewsDTO) -> Result<Self, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    // insert passed values:
    diesel::insert_or_ignore_into(news)
      .values(NewNews::from(new_news))
      .execute(&conn)?;
    // query and return first (most recent updated) item
    let res: News = news.first(&conn)?;
    Ok(res)
  }

  pub fn update(id_to_update: i32, news_updates: NewsDTO) -> Result<Self, ApiError> {
    use crate::schema::news::dsl::*;
    let conn = database::connection()?;
    let pred = id.eq(id_to_update);
    diesel::update(news)
      .filter(&pred)
      .set(NewNews::from(news_updates))
      .execute(&conn)?;
    let res = news.filter(&pred).first(&conn)?;
    Ok(res)
  }

  pub fn _delete(id_to_delete: i32) -> Result<usize, ApiError> {
    use crate::schema::news::dsl::*;

    let conn = database::connection()?;
    let res = diesel::delete(news)
      .filter(id.eq(id_to_delete))
      .execute(&conn)?;
    Ok(res)
  }
}

impl From<News> for PublicNews {
  fn from(news: News) -> Self {
    PublicNews {
      title: news.title,
      link: news.link,
      pub_date: news.pub_date,
      image: news.image,
      image_text: news.image_text,
      content: news.content,
      description: news.description,
      author: news.author,
      hash: news.hash,
      created_at: news.created_at,
      updated_at: news.updated_at,
    }
  }
}

/// "Enrich" provided dto values with a hashid (uuid for now) and return an insertable NewNews object
impl From<NewsDTO> for NewNews {
  fn from(new_news: NewsDTO) -> Self {
    let new_uuid = Uuid::new_v4().to_string();
    NewNews {
      title: new_news.title.unwrap_or_default(),
      link: new_news.link.unwrap_or_default(),
      pub_date: new_news.pub_date.unwrap_or_else(|| Utc::now().naive_utc()),
      image: new_news.image,
      image_text: new_news.image_text,
      content: new_news.content,
      description: new_news.description,
      author: new_news.author,
      hash: new_uuid,
    }
  }
}
