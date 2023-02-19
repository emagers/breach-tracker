CREATE TABLE breach_data (
	id INTEGER PRIMARY KEY NOT NULL,
	date_reported TIMESTAMP NOT NULL,
	organization_name TEXT NOT NULL,
	date_of_breach TIMESTAMP NOT NULL,
	affected_count INTEGER NOT NULL,
	loc INTEGER NOT NULL
);

CREATE TABLE classification (
	id INTEGER PRIMARY KEY NOT NULL,
	breach_data_id INTEGER NOT NULL,
	content TEXT NOT NULL,
	classification_type INTEGER NOT NULL
);