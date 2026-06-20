CREATE TYPE user_role AS ENUM ('admin');

CREATE TABLE users (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL,
	role user_role NOT NULL DEFAULT 'admin',
	email TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL
);
