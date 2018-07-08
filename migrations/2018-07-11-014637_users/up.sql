CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE "user" (
  id uuid primary key default gen_random_uuid(),
  account VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  salt VARCHAR NOT NULL,
  nickname VARCHAR NOT NULL,
  avatar VARCHAR,
  bio VARCHAR,
  signup_time timestamp not null default current_timestamp,
  role smallint not null default 2,
  state smallint not null default 0
);