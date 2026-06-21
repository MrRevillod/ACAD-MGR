CREATE TYPE sex AS ENUM ('H', 'M', 'O');

CREATE TABLE academics (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	rut TEXT NOT NULL UNIQUE,
	names TEXT NOT NULL,
	paternal_surname TEXT NOT NULL,
	maternal_surname TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	orcid TEXT NOT NULL UNIQUE,
	sex sex NOT NULL,
	birth_date DATE NOT NULL,
	joined_at DATE NOT NULL DEFAULT CURRENT_DATE,
	work_position_code TEXT REFERENCES academic_work_positions(code),
	work_position_details TEXT,
	department_id UUID NOT NULL REFERENCES departments(id),
	career_id UUID REFERENCES careers(id),
	uct_working_hours DOUBLE PRECISION NOT NULL,
	acad_category_options_id UUID NOT NULL REFERENCES academic_category_options(id),
	acad_category_hours DOUBLE PRECISION NOT NULL,
	annual_discount_hours DOUBLE PRECISION NOT NULL,
	nationality_code TEXT NOT NULL REFERENCES countries(code),
	city TEXT NOT NULL
);

CREATE TYPE degree_kind AS ENUM (
    'base',
    'advanced'
);

CREATE TABLE degrees (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	academic_id UUID NOT NULL REFERENCES academics(id),
	name TEXT NOT NULL,
	university TEXT NOT NULL,
	obtained_at DATE NOT NULL,
	kind degree_kind NOT NULL,
	country_code TEXT NOT NULL REFERENCES countries(code)
);
