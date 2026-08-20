CREATE TABLE academic_edit_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    academic_id UUID NOT NULL REFERENCES academics(id) ON DELETE CASCADE,
    code TEXT NOT NULL UNIQUE,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_academic_edit_codes_academic ON academic_edit_codes(academic_id);
CREATE INDEX idx_academic_edit_codes_code ON academic_edit_codes(code);
