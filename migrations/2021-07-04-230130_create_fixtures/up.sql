CREATE TABLE fixtures
(
  id         INTEGER        NOT NULL PRIMARY KEY,
  name       VARCHAR        NOT NULL,
  link       VARCHAR UNIQUE NOT NULL,
  team1      VARCHAR        NOT NULL,
  team2      VARCHAR        NOT NULL,
  start_time TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
  rating     INTEGER        NOT NULL DEFAULT 0,
  meta       VARCHAR        NOT NULL,
  analytics  VARCHAR        NOT NULL,
  top_tier   BOOLEAN        NOT NULL DEFAULT true,
  hash       VARCHAR UNIQUE NOT NULL,
  created_at TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
)
