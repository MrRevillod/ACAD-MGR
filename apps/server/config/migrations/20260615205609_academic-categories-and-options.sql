CREATE TYPE academic_option AS ENUM (
    'teaching',
    'research'
);

CREATE TYPE academic_planta AS ENUM (
    'adjunta',
    'permanente'
);

CREATE TABLE academic_categories (
    code    TEXT    PRIMARY KEY,
    name    TEXT    NOT NULL,
    planta  academic_planta  NOT NULL
);

INSERT INTO academic_categories (code, name, planta) VALUES
    -- Planta Permanente
    ('titular_teacher',    'Profesor Titular',    'permanente'),
    ('associated_teacher', 'Profesor Asociado',   'permanente'),
    ('assistant_teacher',  'Profesor Asistente',  'permanente'),
    ('instructor_teacher', 'Profesor Instructor', 'permanente'),
    ('young_doctor',       'Doctor Joven',        'permanente'),
    ('uncategorized',      'Sin Categorizar',     'permanente'),
    -- Planta Adjunta
    ('attached_teacher',    'Profesor Adjunto',     'adjunta'),
    ('attached_instructor', 'Instructor Adjunto',   'adjunta'),
    ('attached_researcher', 'Investigador Adjunto', 'adjunta');

-- Combinaciones válidas según reglamento UCT
CREATE TABLE academic_category_options (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_code TEXT           NOT NULL REFERENCES academic_categories(code),
    option        AcademicOption NOT NULL,
    UNIQUE (category_code, option)
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
