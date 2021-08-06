CREATE TABLE IF NOT EXISTS users(
id uuid primary key NOT NULL,
username VARCHAR(80) NOT NULL,
password_hashed VARCHAR(150) NOT NULL
)