CREATE TYPE work_type AS ENUM (
	'article',
	'book',
	'book-chapter',
	'book-review',
	'conference-abstract',
	'conference-paper',
	'data-paper',
	'dissertation',
	'editorial',
	'erratum',
	'letter',
	'libguide',
	'other',
	'paratext',
	'peer-review',
	'preprint',
	'reference-entry',
	'report',
	'retraction',
	'review',
	'software',
	'software-paper',
	'standard',
	'supplementary-materials'
);

CREATE TYPE journal_kind AS ENUM ('wos', 'scopus');

CREATE TABLE journal_issn (
	id UUID PRIMARY KEY default gen_random_uuid(),
	issn TEXT UNIQUE,
	kind journal_kind NOT NULL
);

CREATE TABLE sources (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	ty TEXT NOT NULL,
	issn TEXT
);

CREATE INDEX idx_sources_openalex_id ON sources(openalex_id);
CREATE INDEX idx_journal_kind ON journal_issn(kind);

CREATE type work_overrides AS (
	title TEXT,
	abstract_text TEXT,
	doi TEXT,
	publication_year SMALLINT,
	is_accepted BOOLEAN,
	is_published BOOLEAN,
	research_line_id UUID
);

CREATE TABLE works (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	title TEXT NOT NULL,
	abstract_text TEXT,
	doi TEXT UNIQUE,
	publication_date DATE,
	publication_year SMALLINT,
	ty work_type NOT NULL,
	lang TEXT NOT NULL,
	is_accepted BOOLEAN NOT NULL DEFAULT FALSE,
	is_published BOOLEAN NOT NULL DEFAULT FALSE,
	source_id UUID REFERENCES sources(id) ON DELETE SET NULL,
	overrides work_overrides NOT NULL DEFAULT ROW(NULL, NULL, NULL, NULL, NULL, NULL, NULL)::work_overrides,
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_works_openalex_id ON works(openalex_id);
CREATE INDEX idx_works_doi ON works(doi);

CREATE TABLE work_topic_scores (
	work_id UUID REFERENCES works(id) ON DELETE CASCADE,
	topic_id UUID REFERENCES topics(id) ON DELETE CASCADE,
	score DOUBLE PRECISION NOT NULL,
	PRIMARY KEY (work_id, topic_id)
);

CREATE INDEX idx_work_topic_scores_work_id ON work_topic_scores(work_id);
CREATE INDEX idx_work_topic_scores_topic_id ON work_topic_scores(topic_id);
CREATE INDEX idx_work_topic_scores_score ON work_topic_scores(score DESC);

CREATE TABLE work_keyword_scores (
	work_id UUID REFERENCES works(id) ON DELETE CASCADE,
	keyword_id UUID REFERENCES keywords(id) ON DELETE CASCADE,
	score DOUBLE PRECISION NOT NULL,
	PRIMARY KEY (work_id, keyword_id)
);

CREATE INDEX idx_work_keyword_scores_work_id ON work_keyword_scores(work_id);
CREATE INDEX idx_work_keyword_scores_keyword_id ON work_keyword_scores(keyword_id);
CREATE INDEX idx_work_keyword_scores_score ON work_keyword_scores(score DESC);
