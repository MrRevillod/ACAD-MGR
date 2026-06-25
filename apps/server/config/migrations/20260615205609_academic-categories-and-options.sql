CREATE TYPE academic_option AS ENUM (
    'teaching',
    'research'
);

CREATE TYPE academic_planta AS ENUM (
    'adjunta',
    'permanente'
);

CREATE TABLE academic_categories (
    id      UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    name    TEXT            NOT NULL,
    planta  academic_planta NOT NULL
);

INSERT INTO academic_categories (name, planta) VALUES
    ('Profesor Titular',     'permanente'),
    ('Profesor Asociado',    'permanente'),
    ('Profesor Asistente',   'permanente'),
    ('Profesor Instructor',  'permanente'),
    ('Doctor Joven',         'permanente'),
    ('Sin Categorizar',      'permanente'),
    ('Profesor Adjunto',     'adjunta'),
    ('Instructor Adjunto',   'adjunta'),
    ('Investigador Adjunto', 'adjunta');

CREATE TABLE academic_category_options (
    id           UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    category_id  UUID            NOT NULL REFERENCES academic_categories(id),
    option       academic_option NOT NULL,
    hours        DOUBLE PRECISION,
    UNIQUE (category_id, option)
);

INSERT INTO academic_category_options (category_id, option, hours) VALUES
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Titular'),     'teaching', 12),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Titular'),     'research', 10),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asociado'),    'teaching', 12),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asociado'),    'research',  8),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asistente'),   'teaching', 14),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asistente'),   'research', 10),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Instructor'),  'teaching', 16),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Adjunto'),     'teaching', 18),
    ((SELECT id FROM academic_categories WHERE name = 'Instructor Adjunto'),   'teaching', 20),

    ((SELECT id FROM academic_categories WHERE name = 'Doctor Joven'),         'teaching', NULL),
    ((SELECT id FROM academic_categories WHERE name = 'Sin Categorizar'),      'teaching', NULL),
    ((SELECT id FROM academic_categories WHERE name = 'Investigador Adjunto'), 'teaching', NULL);
