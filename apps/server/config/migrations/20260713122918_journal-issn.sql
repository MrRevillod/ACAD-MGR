CREATE TYPE journal_kind AS ENUM ('wos', 'scopus');

CREATE TABLE journal_issn (
	id SERIAL PRIMARY KEY,
	issn TEXT UNIQUE,
	eissn TEXT UNIQUE,
	kinds journal_kind[] NOT NULL DEFAULT '{}'
);
