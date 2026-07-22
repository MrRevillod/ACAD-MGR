CREATE TABLE research_lines (
    id    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name  TEXT NOT NULL,
    slug  TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE research_line_mappings (
    research_line_id     UUID NOT NULL REFERENCES research_lines(id) ON DELETE CASCADE,
    subfield_openalex_id TEXT NOT NULL,
    PRIMARY KEY (subfield_openalex_id)
);

CREATE TABLE work_research_line_overrides (
    work_id            UUID NOT NULL REFERENCES works(id) ON DELETE CASCADE,
    research_line_id   UUID NOT NULL REFERENCES research_lines(id),
    PRIMARY KEY (work_id)
);

INSERT INTO research_lines (name, slug) VALUES
    ('Materiales Avanzados y Bioproductos', 'materiales-avanzados'),
    ('Ciencias de la Tierra', 'ciencias-tierra'),
    ('Sostenibilidad', 'sostenibilidad'),
    ('IA, Sistemas Complejos y Modelamiento Matemático', 'ia-sistemas-complejos'),
    ('Educación en Ingeniería', 'educacion-ingenieria'),
    ('Sin Asignar', 'sin-asignar');
