ALTER TYPE work_overrides ADD ATTRIBUTE journal_kind journal_kind;

ALTER TABLE works ALTER COLUMN overrides SET DEFAULT ROW(
	NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
)::work_overrides;