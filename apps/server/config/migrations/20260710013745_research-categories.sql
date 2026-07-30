CREATE TABLE domains (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL
);

CREATE INDEX idx_domains_openalex_id ON domains(openalex_id);

CREATE TABLE fields (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	domain_id UUID REFERENCES domains(id) ON DELETE CASCADE
);

CREATE INDEX idx_fields_openalex_id ON fields(openalex_id);
CREATE INDEX idx_fields_domain_id ON fields(domain_id);

CREATE TABLE research_lines (
	id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL,
	slug TEXT NOT NULL UNIQUE
);

INSERT INTO research_lines (name, slug) VALUES
	('Materiales Avanzados y Bioproductos', 'materiales-avanzados'),
	('Ciencias de la Tierra', 'ciencias-tierra'),
	('Sostenibilidad', 'sostenibilidad'),
	('IA, Sistemas Complejos y Modelamiento Matemático', 'ia-sistemas-complejos'),
	('Educación en Ingeniería', 'educacion-ingenieria'),
	('Sin Asignar', 'sin-asignar');

CREATE TABLE subfields (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	field_id UUID REFERENCES fields(id) ON DELETE CASCADE,
	research_line_id UUID REFERENCES research_lines(id) ON DELETE SET NULL
);

CREATE INDEX idx_subfields_openalex_id ON subfields(openalex_id);
CREATE INDEX idx_subfields_field_id ON subfields(field_id);
CREATE INDEX idx_subfields_research_line_id ON subfields(research_line_id);

CREATE TABLE topics (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL,
	subfield_id UUID REFERENCES subfields(id) ON DELETE CASCADE
);

CREATE INDEX idx_topics_openalex_id ON topics(openalex_id);
CREATE INDEX idx_topics_subfield_id ON topics(subfield_id);

CREATE TABLE keywords (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	openalex_id TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL
);

CREATE INDEX idx_keywords_openalex_id ON keywords(openalex_id);
