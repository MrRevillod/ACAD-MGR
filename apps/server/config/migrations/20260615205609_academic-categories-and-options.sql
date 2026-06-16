CREATE TYPE AcademicOption AS ENUM (
    'teaching',
    'research'
);

CREATE TYPE Planta AS ENUM (
    'Adjunta',
    'Permanente'
);

CREATE TABLE academic_categories (
    code    TEXT    PRIMARY KEY,
    name    TEXT    NOT NULL,
    planta  Planta  NOT NULL
);

INSERT INTO academic_categories (code, name, planta) VALUES
    -- Planta Permanente
    ('titular_teacher',    'Profesor Titular',    'Permanente'),
    ('associated_teacher', 'Profesor Asociado',   'Permanente'),
    ('assistant_teacher',  'Profesor Asistente',  'Permanente'),
    ('instructor_teacher', 'Profesor Instructor', 'Permanente'),
    ('young_doctor',       'Doctor Joven',        'Permanente'),
    ('uncategorized',      'Sin Categorizar',     'Permanente'),
    -- Planta Adjunta
    ('attached_teacher',    'Profesor Adjunto',     'Adjunta'),
    ('attached_instructor', 'Instructor Adjunto',   'Adjunta'),
    ('attached_researcher', 'Investigador Adjunto', 'Adjunta');

-- Combinaciones válidas según reglamento UCT
CREATE TABLE academic_category_options (
    category_code TEXT           NOT NULL REFERENCES academic_categories(code),
    option        AcademicOption NOT NULL,
    PRIMARY KEY (category_code, option)
);

INSERT INTO academic_category_options (category_code, option) VALUES
    -- Permanente: Titular, Asociado, Asistente pueden tener ambas opciones
    ('titular_teacher',    'teaching'),
    ('titular_teacher',    'research'),
    ('associated_teacher', 'teaching'),
    ('associated_teacher', 'research'),
    ('assistant_teacher',  'teaching'),
    ('assistant_teacher',  'research'),
    -- Permanente: Instructor solo teaching (reglamento no contempla research)
    ('instructor_teacher', 'teaching'),
    -- Transitorios: solo teaching
    ('young_doctor',       'teaching'),
    ('uncategorized',      'teaching'),
    -- Adjunta: solo teaching (no tienen opción research según reglamento)
    ('attached_teacher',    'teaching'),
    ('attached_instructor', 'teaching'),
    ('attached_researcher', 'teaching');
