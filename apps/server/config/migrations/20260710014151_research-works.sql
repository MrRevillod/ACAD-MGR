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

CREATE TABLE sources (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	display_name TEXT NOT NULL,
	ty TEXT NOT NULL,
	issn_l TEXT,
	issn TEXT[]
);

CREATE INDEX idx_sources_openalex_id ON sources(openalex_id);

CREATE TABLE works (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	title TEXT NOT NULL,
	abstract TEXT,
	doi TEXT UNIQUE,
	publication_date DATE,
	publication_year SMALLINT,
	ty work_type NOT NULL,
	lang TEXT NOT NULL,
	is_accepted BOOLEAN NOT NULL DEFAULT FALSE,
	is_published BOOLEAN NOT NULL DEFAULT FALSE,
	primary_source_id UUID REFERENCES sources(id) ON DELETE SET NULL
);

CREATE INDEX idx_works_openalex_id ON works(openalex_id);
CREATE INDEX idx_works_doi ON works(doi);

CREATE TABLE work_topics (
	work_id UUID REFERENCES works(id) ON DELETE CASCADE,
	topic_id UUID REFERENCES research_topics(id) ON DELETE CASCADE,
	score DOUBLE PRECISION NOT NULL,
	PRIMARY KEY (work_id, topic_id)
);

CREATE INDEX idx_work_topics_work_id ON work_topics(work_id);
CREATE INDEX idx_work_topics_topic_id ON work_topics(topic_id);

CREATE TABLE work_keywords (
	work_id UUID REFERENCES works(id) ON DELETE CASCADE,
	keyword_id UUID REFERENCES keywords(id) ON DELETE CASCADE,
	score DOUBLE PRECISION NOT NULL,
	PRIMARY KEY (work_id, keyword_id)
);
