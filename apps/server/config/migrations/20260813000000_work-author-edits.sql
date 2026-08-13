ALTER TYPE work_overrides ADD ATTRIBUTE corresponding_orcid TEXT;

ALTER TABLE works
	ALTER COLUMN overrides
	SET DEFAULT ROW(NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL)::work_overrides;
