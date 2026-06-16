-- Add migration script here

CREATE TYPE Sex AS ENUM ('H', 'M', 'O');

CREATE TYPE Planta AS ENUM (
	'adjunta',
	'permanente'
);

CREATE TABLE academics (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	rut TEXT NOT NULL UNIQUE,
	fullname TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	orcid TEXT UNIQUE,
	sex Sex NOT NULL,
	birthday DATE NOT NULL,
	joined_at DATE NOT NULL DEFAULT CURRENT_DATE,
	work_position_code REFERENCES academic_work_positions(code),
	work_position_details TEXT,
	department_id UUID NOT NULL REFERENCES departments(id),
	career_id UUID REFERENCES careers(id),
	uct_working_hours NUMERIC(3, 2) NOT NULL,
	planta Planta NOT NULL,
	acad_category_options_id UUID NOT NULL REFERENCES academic_category_options(id),
	acad_category_hours NUMERIC(3, 2) NOT NULL,
	anual_discount_hours NUMERIC(3, 2) NOT NULL,
	nationality_code TEXT NOT NULL REFERENCES countries(code),
	city TEXT NOT NULL
);

CREATE TYPE DegreeKind AS ENUM (
    'base',
    'advanced'
);

CREATE TABLE degrees (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	academic_id UUID NOT NULL REFERENCES academics(id),
	name TEXT NOT NULL,
	university TEXT NOT NULL,
	obtained_at DATE NOT NULL,
	kind DegreeKind NOT NULL,
	country_code TEXT NOT NULL REFERENCES countries(code)
);
