CREATE TABLE academic_work_positions (
	code TEXT PRIMARY KEY,
	name TEXT NOT NULL
);

INSERT INTO academic_work_positions (code, name) VALUES
('uknown', 'Desconocido'),
('docente', 'Docente'),
('jefe_carrera', 'Jefe(a) de Carrera'),
('director_departamento', 'Director(a) de Departamento'),
('ceda', 'CEDA'),
('director', 'Director(a)'),
('vicedecano_decano_interino', 'Vicedecano(a) - Decano(a) Interino(a)'),
('jefe_carrera_ceda', 'Jefe(a) de Carrera - CEDA'),
('docente_reemplazo', 'Docente de Reemplazo'),
('director_magister', 'Director Magister');
