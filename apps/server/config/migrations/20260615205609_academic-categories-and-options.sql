CREATE TYPE academic_option AS ENUM (
    'teaching',
    'research'
);

CREATE TYPE academic_planta AS ENUM (
    'adjunta',
    'permanente'
);

CREATE TABLE academic_categories (
    id    	UUID    PRIMARY KEY default gen_random_uuid(),
    name    TEXT    NOT NULL,
    planta  academic_planta  NOT NULL
);

INSERT INTO academic_categories (name, planta) VALUES
    -- Planta Permanente
    ('Profesor Titular',    'permanente'),
    ('Profesor Asociado',   'permanente'),
    ('Profesor Asistente',  'permanente'),
    ('Profesor Instructor', 'permanente'),
    ('Doctor Joven',        'permanente'),
    ('Sin Categorizar',     'permanente'),

    -- Planta Adjunta
    ('Profesor Adjunto',     'adjunta'),
    ('Instructor Adjunto',   'adjunta'),
    ('Investigador Adjunto', 'adjunta');

-- Combinaciones válidas según reglamento UCT
CREATE TABLE academic_category_options (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_id   UUID NOT NULL REFERENCES academic_categories(id),
    option        academic_option NOT NULL,
    UNIQUE (category_id, option)
);

INSERT INTO academic_category_options (category_id, option) VALUES
    -- Permanente: Titular, Asociado, Asistente pueden tener ambas opciones
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Titular'), 'teaching'),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Titular'), 'research'),

    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asociado'), 'teaching'),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asociado'), 'research'),

    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asistente'), 'teaching'),
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Asistente'), 'research'),

    -- Permanente: Instructor solo teaching (reglamento no contempla research)
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Instructor'), 'teaching'),

    -- Transitorios: solo teaching
    ((SELECT id FROM academic_categories WHERE name = 'Doctor Joven'), 'teaching'),
    ((SELECT id FROM academic_categories WHERE name = 'Sin Categorizar'), 'teaching'),

    -- Adjunta: solo teaching (no tienen opción research según reglamento)
    ((SELECT id FROM academic_categories WHERE name = 'Profesor Adjunto'), 'teaching'),
    ((SELECT id FROM academic_categories WHERE name = 'Instructor Adjunto'), 'teaching'),
    ((SELECT id FROM academic_categories WHERE name = 'Investigador Adjunto'), 'teaching');
