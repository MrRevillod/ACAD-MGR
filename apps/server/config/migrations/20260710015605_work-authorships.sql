CREATE TYPE authorship_position AS ENUM (
	'first',
	'middle',
	'last'
);

CREATE TABLE work_authorships (
	work_id UUID NOT NULL REFERENCES works(id) ON DELETE CASCADE,
	orcid TEXT NOT NULL,
	name TEXT NOT NULL,
	is_external BOOLEAN NOT NULL DEFAULT FALSE,
	is_corresponding BOOLEAN NOT NULL DEFAULT FALSE,
	affiliations TEXT[] NOT NULL DEFAULT '{}',
	position authorship_position NOT NULL,
	PRIMARY KEY (work_id, orcid)
);
