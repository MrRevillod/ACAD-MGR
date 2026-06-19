-- Add migration script here

CREATE TABLE faculties (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL
);

INSERT INTO faculties (name) VALUES ('Facultad de Ingeniería');

CREATE TABLE departments (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL,
	faculty_id TEXT NOT NULL REFERENCES faculties(id)
);

INSERT INTO departments (name, faculty_id) VALUES
('Ciencias Matemáticas y Físicas', (SELECT id FROM faculties WHERE name = 'Facultad de Ingeniería')),
('Obras Civiles y Geología', (SELECT id FROM faculties WHERE name = 'Facultad de Ingeniería')),
('Procesos Industriales', (SELECT id FROM faculties WHERE name = 'Facultad de Ingeniería')),
('Ingeniería Informática', (SELECT id FROM faculties WHERE name = 'Facultad de Ingeniería'));

CREATE TABLE careers (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL,
	department_id UUID NOT NULL REFERENCES departments(id)
);

INSERT INTO careers (name, department_id) VALUES
('Ingeniería en Construcción', (SELECT id FROM departments WHERE name = 'Obras Civiles y Geología')),
('Geología', (SELECT id FROM departments WHERE name = 'Obras Civiles y Geología')),
('Ingeniería Civil en Obras Civiles', (SELECT id FROM departments WHERE name = 'Obras Civiles y Geología')),
('Ingeniería Civil Geológica', (SELECT id FROM departments WHERE name = 'Obras Civiles y Geología')),

('Ingeniería Civil Industrial', (SELECT id FROM departments WHERE name = 'Procesos Industriales')),
('Ingeniería Eléctrica', (SELECT id FROM departments WHERE name = 'Procesos Industriales')),
('Ingeniería Civil Química', (SELECT id FROM departments WHERE name = 'Procesos Industriales')),
('Ingeniería Civil Ambiental', (SELECT id FROM departments WHERE name = 'Procesos Industriales')),
('Ingeniería Civil Plan Común', (SELECT id FROM departments WHERE name = 'Procesos Industriales')),

('Ingeniería Civil en Informática', (SELECT id FROM departments WHERE name = 'Ingeniería Informática'));
