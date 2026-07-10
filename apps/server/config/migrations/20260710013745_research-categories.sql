CREATE TABLE research_domains (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL
);

CREATE INDEX idx_research_domains_openalex_id ON research_domains(openalex_id);

CREATE TABLE research_fields (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	domain_id UUID REFERENCES research_domains(id) ON DELETE CASCADE
);

CREATE INDEX idx_research_fields_openalex_id ON research_fields(openalex_id);

CREATE TABLE research_subfields (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	field_id UUID REFERENCES research_fields(id) ON DELETE CASCADE
);

CREATE INDEX idx_research_subfields_openalex_id ON research_subfields(openalex_id);

CREATE TABLE research_topics (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	subfield_id UUID REFERENCES research_subfields(id) ON DELETE CASCADE
);

CREATE INDEX idx_research_topics_openalex_id ON research_topics(openalex_id);

CREATE TABLE keywords (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL
);

CREATE INDEX idx_keywords_openalex_id ON keywords(openalex_id);
