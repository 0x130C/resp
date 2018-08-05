CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS user_profile (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    avatar VARCHAR,
    nickname VARCHAR,
    bio VARCHAR
);

CREATE TABLE "users" (
  id uuid primary key default gen_random_uuid(),
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  salt VARCHAR NOT NULL,
  actived_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  role smallint not null default 2,
  state smallint not null default 0,
  profile_id  uuid REFERENCES user_profile(id) ON UPDATE CASCADE
);

