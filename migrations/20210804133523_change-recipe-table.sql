CREATE TABLE IF NOT EXISTS recipes(
id uuid primary key NOT NULL,
title TEXT NOT NULL,
content TEXT NOT NULL,
published BOOLEAN DEFAULT FALSE
)