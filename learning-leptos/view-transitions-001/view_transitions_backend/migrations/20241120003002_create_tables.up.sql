CREATE TABLE blogs (
  id TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  cover_image_url TEXT,
  banner_image_url TEXT,
  source_link_url TEXT NOT NULL,
  tags TEXT NOT NULL
) STRICT;
