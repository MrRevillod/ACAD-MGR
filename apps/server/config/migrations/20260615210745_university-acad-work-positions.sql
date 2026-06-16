-- Add migration script here

CREATE TABLE academic_work_positions (
	code TEXT PRIMARY KEY,
	name TEXT NOT NULL
);

INSERT INTO academic_work_positions (code, name) VALUES
('docente', 'Docente'),
('jefe_carrera', 'Jefa(e) de Carrera'),
('director_depto', 'Director(a) de Departamento'),
('ceda', 'CEDA'),
('director', 'Director(a)'),
('vice_decano_interino', 'Vicedecano(a) - Decano(a) Interino(a)'),
('jefe_carrera_i_ceda', 'Jefa(e) de Carrera - CEDA'),
('docente_reemplazo', 'Docente de Reemplazo'),
('director_magister', 'Director Magister');
