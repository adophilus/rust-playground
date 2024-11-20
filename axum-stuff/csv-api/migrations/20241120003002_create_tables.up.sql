CREATE TABLE blogs (
  id TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  link TEXT NOT NULL,
  image_url TEXT NOT NULL,
  topic TEXT NOT NULL,
  info TEXT NOT NULL,
  preprocessed TEXT NOT NULL
) STRICT;
