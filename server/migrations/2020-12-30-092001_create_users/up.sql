BEGIN;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);
-- need to clear the table as we going to add new NOT NULL column
TRUNCATE TABLE articles CASCADE;
ALTER TABLE articles
ADD COLUMN author INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE;
COMMIT;