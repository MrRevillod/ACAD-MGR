CREATE TABLE academic_work_positions (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL UNIQUE
);

INSERT INTO academic_work_positions (name) VALUES
('Desconocido'),
('Docente'),
('Jefe(a) de Carrera'),
('Director(a) de Departamento'),
('CEDA'),
('Director(a)'),
('Vicedecano(a) - Decano(a) Interino(a)'),
('Jefe(a) de Carrera - CEDA'),
('Docente de Reemplazo'),
('Director Magister');
