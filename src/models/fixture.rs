use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database;
use crate::errors::ApiError;
use crate::schema::fixtures;

/// Representation of fixture
///
/// Results from data store are represented as vector of `Fixture` structs.
/// contents of the string. It has a close relationship with its borrowed
/// counterpart, the primitive [`str`].
///
/// # Examples
///
/// You can create a `String` from [a literal string][`str`] with [`String::from`]:
///
/// [`String::from`]: From::from
///
/// ```
/// let hello = String::from("Hello, world!");
/// ```
///
/// You can append a [`char`] to a `String` with the [`push`] method, and
/// append a [`&str`] with the [`push_str`] method:
#[derive(Serialize, Deserialize, AsChangeset, Queryable)]
#[table_name = "fixtures"]
pub struct Fixture {
  pub id: i32,
  pub name: String,
  pub link: String,
  pub team1: String,
  pub team2: String,
  pub start_time: NaiveDateTime,
  pub rating: i32,
  pub meta: String,
  pub analytics: String,
  pub top_tier: bool,
  pub hash: String,
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

/// use new to send object to data store (sqlite)
#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "fixtures"]
pub struct NewFixture {
  pub name: String,
  pub link: String,
  pub team1: String,
  pub team2: String,
  pub start_time: NaiveDateTime,
  pub rating: i32,
  pub meta: String,
  pub analytics: String,
  pub top_tier: bool,
  pub hash: String,
}

/// use new to send object to data store (sqlite)
#[derive(Deserialize, Serialize)]
pub struct PublicFixture {
  pub name: String,
  pub link: String,
  pub team1: String,
  pub team2: String,
  pub start_time: NaiveDateTime,
  pub rating: i32,
  pub meta: String,
  pub analytics: String,
  pub top_tier: bool,
  pub hash: String,
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

/// use dto to send data between modules
#[derive(Deserialize, Serialize)]
pub struct FixtureDTO {
  pub id: Option<i32>,
  pub name: Option<String>,
  pub link: Option<String>,
  pub team1: Option<String>,
  pub team2: Option<String>,
  pub start_time: Option<NaiveDateTime>,
  pub rating: Option<i32>,
  pub meta: Option<String>,
  pub analytics: Option<String>,
  pub top_tier: Option<bool>,
  pub hash: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
}

impl Fixture {
  pub fn all() -> Result<Vec<Self>, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let res: Vec<Fixture> = fixtures.load::<Fixture>(&conn)?;
    Ok(res)
  }

  pub fn all_public() -> Result<Vec<PublicFixture>, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let db_result: Vec<Fixture> = fixtures.load::<Fixture>(&conn)?;
    let res: Vec<PublicFixture> = db_result.into_iter().map(PublicFixture::from).collect();
    Ok(res)
  }

  pub fn filtered_public(
    limit_to: Option<i64>,
    is_top_tier: Option<bool>,
  ) -> Result<Vec<PublicFixture>, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let limit = limit_to.unwrap_or(10);
    let filter_top_tier = is_top_tier.unwrap_or(true);
    let db_result: Vec<Fixture> = fixtures
      .limit(limit)
      .filter(top_tier.eq(filter_top_tier))
      .load::<Fixture>(&conn)?;
    let res: Vec<PublicFixture> = db_result.into_iter().map(PublicFixture::from).collect();
    Ok(res)
  }

  pub fn get(id_to_get: i32) -> Result<Self, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let res = fixtures.filter(id.eq(id_to_get)).first(&conn)?;
    Ok(res)
  }

  pub fn find(hash_to_find: String) -> Result<Self, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let res = fixtures.filter(hash.eq(hash_to_find)).first(&conn)?;
    Ok(res)
  }

  pub fn find_public(hash_to_find: String) -> Result<PublicFixture, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let db_result: Fixture = fixtures.filter(hash.eq(hash_to_find)).first(&conn)?;
    let res: PublicFixture = PublicFixture::from(db_result);
    Ok(res)
  }

  /// create many new fixtures
  ///
  /// # Arguments
  ///
  /// * `incoming`: vector/array of new fixtures to create
  ///
  /// returns: Result<usize, ApiError>
  ///
  /// # Examples
  ///
  /// ```
  /// let mut fixtures: Vec<FixtureDTO> = Vec::new();
  /// fixtures.push(Fixture {/*...*/});
  /// fixtures.push(Fixture {/*...*/});
  ///
  /// let inserted_count = Fixture::create_many(fixtures);
  /// assert!(inserted_count,
  /// ```
  pub fn create(new_fixture: FixtureDTO) -> Result<Self, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    // insert passed values:
    diesel::insert_or_ignore_into(fixtures)
      .values(NewFixture::from(new_fixture))
      .execute(&conn)?;
    // query and return first (most recent updated) item
    let res: Fixture = fixtures.first(&conn)?;
    Ok(res)
  }

  /// create many new fixtures.
  /// inserts are done by looping over the provided vector and inserting them one by one.
  /// sqlite doesnt support bulk inserts.
  ///
  /// # Arguments
  ///
  /// * `incoming`: vector/array of new fixtures to create
  ///
  /// returns: Result<usize, ApiError>
  ///
  /// # Examples
  ///
  /// ```
  /// let mut fixtures: Vec<FixtureDTO> = Vec::new();
  /// fixtures.push(Fixture {/*...*/});
  /// fixtures.push(Fixture {/*...*/});
  ///
  /// let inserted_count = Fixture::create_many(fixtures);
  /// assert!(inserted_count,
  /// ```
  pub fn create_many(incoming: Vec<FixtureDTO>) -> Result<usize, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let mut inserted_count = 0;
    for new_fixture in incoming.into_iter() {
      diesel::insert_or_ignore_into(fixtures)
        .values(NewFixture::from(new_fixture))
        .execute(&conn)?;
      inserted_count = inserted_count + 1;
    }
    Ok(inserted_count)
  }

  /// TODO: I'm unsure if we will ever need this?!
  ///       if so, it would override hash-value for sure
  pub fn _update(id_to_update: i32, fixture_updates: FixtureDTO) -> Result<Self, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let pred = id.eq(id_to_update);
    diesel::update(fixtures)
      .filter(&pred)
      .set(NewFixture::from(fixture_updates))
      .execute(&conn)?;
    let res = fixtures.filter(&pred).first(&conn)?;
    Ok(res)
  }

  pub fn _delete(id_to_delete: i32) -> Result<usize, ApiError> {
    use crate::schema::fixtures::dsl::*;
    let conn = database::connection()?;
    let res = diesel::delete(fixtures.filter(id.eq(id_to_delete))).execute(&conn)?;
    Ok(res)
  }
}

impl From<Fixture> for PublicFixture {
  fn from(fixture: Fixture) -> Self {
    PublicFixture {
      name: fixture.name,
      link: fixture.link,
      team1: fixture.team1,
      team2: fixture.team2,
      start_time: fixture.start_time,
      rating: fixture.rating,
      meta: fixture.meta,
      analytics: fixture.analytics,
      top_tier: fixture.top_tier,
      hash: fixture.hash,
      created_at: fixture.created_at,
      updated_at: fixture.updated_at,
    }
  }
}

impl From<Fixture> for FixtureDTO {
  fn from(fixture: Fixture) -> Self {
    FixtureDTO {
      id: None,
      name: Some(fixture.name),
      link: Some(fixture.link),
      team1: Some(fixture.team1),
      team2: Some(fixture.team2),
      start_time: Some(fixture.start_time),
      rating: Some(fixture.rating),
      meta: Some(fixture.meta),
      analytics: Some(fixture.analytics),
      top_tier: Some(fixture.top_tier),
      hash: Some(fixture.hash),
      created_at: Some(fixture.created_at),
      updated_at: Some(fixture.updated_at.unwrap()),
    }
  }
}

/// "Enrich" provided dto values with a hashid (uuid for now) and return an insertable NewFixture object
impl From<FixtureDTO> for NewFixture {
  fn from(new_fixture: FixtureDTO) -> Self {
    let new_uuid = Uuid::new_v4().to_string();

    NewFixture {
      name: new_fixture.name.unwrap_or_default(),
      link: new_fixture.link.unwrap_or_default(),
      team1: new_fixture.team1.unwrap_or_default(),
      team2: new_fixture.team2.unwrap_or_default(),
      start_time: new_fixture
        .start_time
        .unwrap_or_else(|| Utc::now().naive_utc()),
      rating: new_fixture.rating.unwrap_or_default(),
      meta: new_fixture.meta.unwrap_or_default(),
      analytics: new_fixture.analytics.unwrap_or_default(),
      top_tier: new_fixture.top_tier.unwrap_or_default(),
      hash: new_uuid,
    }
  }
}
