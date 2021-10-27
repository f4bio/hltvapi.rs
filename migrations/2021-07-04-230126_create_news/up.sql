CREATE TABLE news
(
  id          INTEGER        NOT NULL PRIMARY KEY,
  title       VARCHAR        NOT NULL,
  link        VARCHAR UNIQUE NOT NULL,
  pub_date    TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
  image       VARCHAR,
  image_text  VARCHAR,
  content     VARCHAR,
  description VARCHAR,
  author      VARCHAR,
  hash        VARCHAR UNIQUE NOT NULL,
  created_at  TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at  TIMESTAMP
)
