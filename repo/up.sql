SET statement_timeout = 0;
SET lock_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;

\connect youareso

SET statement_timeout = 0;
SET lock_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;
COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';
SET search_path = public, pg_catalog;
SET default_tablespace = '';
SET default_with_oids = false;

CREATE TABLE "user" (
  id BIGSERIAL PRIMARY KEY,
  username character varying(100) NOT NULL,
  telegram_id BIGINT NOT NULL
);

CREATE TABLE subject (
  id BIGSERIAL PRIMARY KEY,
  value character varying(100) NOT NULL
);

CREATE TABLE adjective (
  id BIGSERIAL PRIMARY KEY,
  value character varying(100) NOT NULL
);

CREATE TABLE joke (
  id BIGSERIAL PRIMARY KEY,
  text character varying(2048) NOT NULL,
  author_id BIGINT NOT NULL REFERENCES "user" (id),
  subject_id BIGINT NOT NULL REFERENCES subject (id),
  adjective_id BIGINT NOT NULL REFERENCES adjective (id)
);
