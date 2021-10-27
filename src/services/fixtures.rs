extern crate reqwest;
extern crate scraper;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use icalendar::{Calendar, Component};
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, info};

use crate::constants::{
  BASE_URL, FILE_STORAGE_LOCATION, FIXTURES_CALENDAR_FILENAME, FIXTURES_URL, FIXTURES_URL_TOP_TIER,
};
use crate::models::fixture::{Fixture, FixtureDTO};
use crate::services::do_fetch;

fn build_fixture(raw_html: ElementRef) -> Option<FixtureDTO> {
  // # ------ IS EMPTY
  let is_empty_selector = Selector::parse("div.matchInfoEmpty").unwrap();
  let is_empty_element = raw_html.select(&is_empty_selector).next();
  if is_empty_element.is_some() {
    return None;
  };

  // # ------ NAME
  let name_selector = Selector::parse("div.matchEventName").unwrap();
  let name_element = raw_html.select(&name_selector).next().unwrap();
  let name = name_element.inner_html();
  // debug!("raw_html#name: {}", name);

  // # ------ TEAMS
  let teams_selector = Selector::parse("div.matchTeamName").unwrap();
  let mut teams_iter = raw_html.select(&teams_selector);
  let team1_element = teams_iter.next();
  let team1 = match team1_element {
    Some(t) => t.inner_html(),
    None => String::from("TBD"),
  };
  // debug!("team1: {}", team1);
  let team2_element = teams_iter.next();
  let team2 = match team2_element {
    Some(t) => t.inner_html(),
    None => String::from("TBD"),
  };
  // debug!("team2: {}", team2);

  // # ------ TIME
  let time_selector = Selector::parse("div.matchTime").unwrap();
  let time_element = raw_html.select(&time_selector).next();
  let time_string = time_element.unwrap().inner_html();
  let start_time_obj: DateTime<Utc> = if time_string == "LIVE" {
    Utc::now()
  } else {
    let _ts = time_element.unwrap().value().attr("data-unix").unwrap();
    let _millis = i64::from_str(_ts).unwrap();
    Utc.timestamp_millis(_millis)
  };
  let start_time: NaiveDateTime = start_time_obj.naive_utc();
  // debug!("start_time: {}", start_time);

  // # ------ RATING
  let rating_selector = Selector::parse("div.matchRating").unwrap();
  let rating_element = raw_html.select(&rating_selector).next().unwrap();
  let rating_stars_selector = Selector::parse("i.faded").unwrap();
  let rating = (5 - rating_element.select(&rating_stars_selector).count()) as i32;

  // # ------ META
  let meta_selector = Selector::parse("div.matchMeta").unwrap();
  let meta_element = raw_html.select(&meta_selector).next();
  let meta = meta_element?.inner_html();
  // debug!("raw_html#meta: {}", meta.clone().unwrap_or(String::from("> No meta found <")));

  // # ------ LINK
  let link_selector = Selector::parse("a.match").unwrap();
  let link_element = raw_html.select(&link_selector).next().unwrap();

  let link = String::from(link_element.value().attr("href").unwrap());
  // debug!("raw_html#link: {}", link);

  // # ------ ANALYTICS
  let analytics_selector = Selector::parse("a.matchAnalytics").unwrap();
  let analytics_element = raw_html.select(&analytics_selector).next();
  let analytics = String::from(analytics_element?.value().attr("href").unwrap());

  Some(FixtureDTO {
    id: None,
    name: Some(name),
    link: Some(link),
    team1: Some(team1),
    team2: Some(team2),
    start_time: Some(start_time),
    rating: Some(rating),
    meta: Some(meta),
    analytics: Some(analytics),
    top_tier: Some(false),
    hash: None,
    created_at: None,
    updated_at: None,
  })
}

#[test]
fn test_build_fixture() {
  use std::fs;

  let path = Path::new("tests").join("scraped.fixtures_toptier.example.html");
  let content = fs::read_to_string(path).unwrap();

  let fragment = Html::parse_document(&content);
  let current_fixtures = Selector::parse("div.liveMatch,div.upcomingMatch").unwrap();

  for fix in fragment.select(&current_fixtures) {
    let new_fixture = build_fixture(fix);
    // insta::assert_yaml_snapshot!(new_fixture);
    assert_eq!(
      new_fixture.unwrap().name.unwrap(),
      "ESL Pro League Season 14"
    );

    break;
  }
}

fn update_fixture_calendar() {
  info!("updating fixtures calendar...");

  let mut fixture_calendar = Calendar::new();
  let all_fixtures = Fixture::all().unwrap();

  for fix in all_fixtures {
    let summary = format!(
      "{} - {} vs. {} ({})",
      fix.name, fix.team1, fix.team2, fix.meta
    );
    let mut rating_stars = String::with_capacity(5);
    for cnt in 0..5 {
      if cnt >= fix.rating {
        rating_stars = rating_stars + "☆";
      } else {
        rating_stars = rating_stars + "★";
      }
    }
    let link = format!("{}{}", BASE_URL, fix.link);
    let description = format!(
      "rating: {} ({}/5)\nlink: {}\nanalytics: {}\n",
      rating_stars, fix.rating, link, fix.analytics
    );
    let starts = fix.start_time;
    let ends = fix.start_time + Duration::hours(2);

    let event = icalendar::Event::new()
      .uid(fix.hash.as_str())
      .summary(summary.as_str())
      .description(description.as_str())
      .starts(starts)
      .ends(ends)
      .done();
    fixture_calendar.push(event);
  }
  let cal = fixture_calendar.to_string();

  let path = Path::new(FILE_STORAGE_LOCATION).join(FIXTURES_CALENDAR_FILENAME);
  let mut file = File::create(path.clone()).unwrap();
  file.write_all(cal.as_ref()).unwrap();

  info!("fixtures calendar stored at: {}!", path.to_str().unwrap());
}

async fn do_scrape(url: &str) -> Vec<FixtureDTO> {
  let mut fixtures: Vec<FixtureDTO> = Vec::new();
  let body: String = do_fetch(url).await.unwrap();
  let fragment = Html::parse_document(&body);
  let current_fixtures = Selector::parse("div.upcomingMatch").unwrap();
  for fix in fragment.select(&current_fixtures) {
    let new_fixture = build_fixture(fix);
    if new_fixture.is_some() {
      fixtures.push(new_fixture.unwrap());
    }
  }
  return fixtures;
}

pub async fn scrape_fixtures() {
  info!("scraping fixtures...");

  let mut fixtures: Vec<FixtureDTO> = Vec::new();
  fixtures.append(&mut do_scrape(FIXTURES_URL).await);
  fixtures.append(&mut do_scrape(FIXTURES_URL_TOP_TIER).await);

  debug!("adding matches to database...");
  let inserted_fixture = Fixture::create_many(fixtures);
  if inserted_fixture.is_err() {
    info!("fixture already exists, skipping...",);
  }
  info!(
    "inserted '{}' new fixtures",
    inserted_fixture.as_ref().unwrap()
  );

  debug!("adding matches to calendar...");
  let start = Instant::now();
  update_fixture_calendar();
  let duration = start.elapsed();
  debug!(
    "> Time elapsed in update_fixture_calendar() is: {:?}",
    duration
  );

  info!("fixtures scraped!");
}
