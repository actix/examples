DROP DATABASE IF EXISTS testing_db;

CREATE USER test_user WITH PASSWORD 'testing';

CREATE DATABASE testing_db OWNER test_user;

\connect testing_db;

DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;


CREATE TABLE testing.users (
	id  BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (username)
);
