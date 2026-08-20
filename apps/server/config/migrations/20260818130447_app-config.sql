CREATE TABLE app_config (
    id   INT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL
);

INSERT INTO app_config (id, data) VALUES (1, '{"jce_max": 42.5}');
