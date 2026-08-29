CREATE UNIQUE INDEX degrees_single_superior_idx
    ON degrees (academic_id)
    WHERE kind IN ('magister', 'doctor');