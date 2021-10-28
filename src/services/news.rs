extern crate reqwest;
extern crate scraper;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::thread;

use chrono::{NaiveDateTime, TimeZone, Utc};
use itconfig::get_env_or_default;
use rss::extension::syndication::SyndicationExtension;
use rss::{Channel, ChannelBuilder, Enclosure, EnclosureBuilder, Guid, Image, ImageBuilder, Item};
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, error, info};

use crate::constants::{BASE_URL, FILE_STORAGE_LOCATION, NEWS_FEED_FILENAME, NEWS_URL};
use crate::models::news::{News, NewsDTO};
use crate::services::do_fetch;

fn build_news(html_content: String) -> Option<NewsDTO> {
  // debug!("build_news: {:?}", html_content);

  let fragment = Html::parse_document(&html_content);
  let news_item_selector = Selector::parse("article.newsitem").unwrap();
  let news_item_element = fragment.select(&news_item_selector).next();
  if news_item_element.is_none() {
    error!(
      "selector 'article.newsitem' not found in html '{:?}'...",
      html_content
    );
    return None;
  };
  let raw_html: ElementRef = news_item_element.unwrap();

  // # ------ Title
  let title_selector = Selector::parse("h1.headline").unwrap();
  let title = match raw_html.select(&title_selector).next() {
    Some(tit) => tit.inner_html(),
    None => String::from("<TITLE>"),
  };
  debug!("build_news#title: {}", title);

  // # ------ LINK
  // og:url
  let link_selector = Selector::parse(r#"meta[property="og:url"]"#).unwrap();
  let link = match fragment.select(&link_selector).next() {
    Some(le) => {
      String::from(le.value().attr("content").unwrap()).replace("https://www.hltv.org", "")
    }
    None => String::from("<LINK>"),
  };
  debug!("build_news#link: {}", link);

  // # ------ PUB_DATE
  // let pub_date: NaiveDateTime = start_time_obj.naive_utc();
  let pub_date_selector = Selector::parse("div.date").unwrap();
  let pub_date_element = raw_html.select(&pub_date_selector).next().unwrap();
  let pub_date_ts = pub_date_element.value().attr("data-unix").unwrap();
  let pub_date_millis = i64::from_str(pub_date_ts).unwrap();
  let pub_date_obj = Utc.timestamp_millis(pub_date_millis);
  let pub_date: NaiveDateTime = pub_date_obj.naive_utc();
  debug!("build_news#pub_date: {}", pub_date);

  // # ------ AUTHOR
  let author_selector = Selector::parse("a.authorName>span").unwrap();
  let author = match fragment.select(&author_selector).next() {
    Some(ae) => ae.inner_html(),
    None => String::from("<AUTHOR>"),
  };

  // # ------ IMAGE
  let image_selector = Selector::parse("div.image-con>img.image").unwrap();
  let image = match raw_html.select(&image_selector).next() {
    Some(img) => String::from(img.value().attr("src").unwrap()),
    None => String::from("<IMAGE>"),
  };

  // # ------ IMAGETEXT
  let image_text_selector = Selector::parse("div.imagetext").unwrap();
  let image_text = match raw_html.select(&image_text_selector).next() {
    Some(img_txt) => img_txt.html(),
    None => String::from("<IMAGE TEXT>"),
  };

  // # ------ CONTENT
  let content_selector = Selector::parse("div.newstext-con>p.news-block").unwrap();
  let mut content_collector = String::new();
  for content_block in raw_html.select(&content_selector) {
    debug!("content block: {}", content_block.html().as_str());
    content_collector.push_str(content_block.html().as_str());
  }

  // # ------ DESCRIPTION
  let description_selector = Selector::parse("p.headertext").unwrap();
  let description = match raw_html.select(&description_selector).next() {
    Some(desc) => desc.inner_html(),
    None => String::from("<DESCRIPTION>"),
  };

  Some(NewsDTO {
    id: None,
    title: Some(title),
    link: Some(link),
    pub_date: Some(pub_date),
    image: Some(image),
    image_text: Some(image_text),
    content: Some(content_collector),
    description: Some(description),
    author: Some(author),
    hash: None,
    created_at: None,
    updated_at: None,
  })
}

#[test]
fn test_build_news() {
  use std::fs;

  let path = Path::new("tests").join("scraped.news_item.example.html");
  let content = fs::read_to_string(path).unwrap();

  let item = build_news(content);

  // insta::assert_yaml_snapshot!(item.unwrap().title);
  assert_eq!(
    item.unwrap().title.unwrap(),
    "EliGE shines as Liquid hammer NIP in ESL Pro League opener"
  );
}

fn update_news_feed() {
  info!("updating news feed...");
  let app_base_url: String = get_env_or_default("PUBLIC_BASE_URL", "https://hltvapi.f4b.io");

  let image: Image = ImageBuilder::default()
    .url("https://hltvapi.f4b.io/logo.png")
    .title("hltvapi logo")
    .build();
  let mut channel: Channel = ChannelBuilder::default()
    .title("Unofficial hltv.org News")
    .link(&app_base_url)
    .description("An unofficial hltv.org RSS news-feed.")
    .image(image)
    .build();
  let all_news = News::all().unwrap();
  let mut news_items: Vec<Item> = vec![];

  for ne in all_news {
    let guid: Guid = Guid {
      value: ne.hash,
      permalink: true,
    };

    let enclosure: Enclosure = EnclosureBuilder::default()
      .mime_type(mime::IMAGE_JPEG.to_string())
      .url(ne.image.unwrap())
      .build();

    // push items into channel
    news_items.push(Item {
      title: Some(ne.title),
      link: Some(format!("{}{}", &app_base_url, ne.link)),
      description: ne.description,
      author: ne.author,
      categories: vec![],
      comments: None,
      guid: Some(guid),
      enclosure: Some(enclosure),
      pub_date: Some(ne.pub_date.to_string()),
      source: None,
      content: ne.content,
      extensions: Default::default(),
      itunes_ext: None,
      dublin_core_ext: None,
    });
  }
  // info!("news items: {:?}", news_items);
  channel.set_syndication_ext(SyndicationExtension::default());
  channel.set_last_build_date(Utc::now().to_rfc2822());
  channel.set_items(news_items);
  let news_feed_rss = channel.to_string();

  let news_feed_rss_path = Path::new(FILE_STORAGE_LOCATION).join(NEWS_FEED_FILENAME);
  let mut news_feed_rss_file = File::create(news_feed_rss_path.clone()).unwrap();
  news_feed_rss_file
    .write_all(news_feed_rss.as_ref())
    .unwrap();

  // let mut opml = OPML::default();
  // let app_url: String = get_env_or_default("APP_SERVER_URL", "http://localhost:1337");
  // let opml_url = format!("{}/news/feed.xml", app_url);
  //
  // opml.head = Some(Head {
  //   title: Some("Rust Feeds".to_string()),
  //   ..Head::default()
  // });
  // opml.add_feed("hltv.org News", opml_url.as_str());
  //
  // let xml = opml.to_string().unwrap();

  info!(
    "news feed rss stored at: {}!",
    news_feed_rss_path.to_str().unwrap()
  );
}

pub async fn scrape_news() {
  info!("scraping base news...");

  let body: String = do_fetch(NEWS_URL).await.unwrap();
  let fragment = Html::parse_document(&body);
  let mut url_list: Vec<String> = Vec::with_capacity(5);

  let news_lines = Selector::parse("a.newsline").unwrap();
  for news_line in fragment.select(&news_lines) {
    let link = String::from(news_line.value().attr("href").unwrap());
    url_list.push(link);
    if url_list.len() >= 5 {
      break;
    }
  }

  for url_path in url_list {
    thread::spawn(move || {
      let resp = reqwest::blocking::get(format!("{}{}", BASE_URL, url_path));
      let body = resp.unwrap().text().unwrap();

      let news_item = build_news(body);
      let inserted_news = News::create(news_item.unwrap());
      if inserted_news.is_err() {
        info!("news already exists, skipping...");
      }
    });
  }

  debug!("adding news to feed...");
  update_news_feed();

  info!("news scraped!");
}
