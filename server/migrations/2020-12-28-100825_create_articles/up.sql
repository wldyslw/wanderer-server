CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  slug TEXT NOT NULL UNIQUE,
  title TEXT NOT NULL,
  title_image TEXT NOT NULL,
  description TEXT NOT NULL,
  body TEXT NOT NULL,
  tag_list TEXT [] NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  favorites_count INTEGER NOT NULL DEFAULT 0 CHECK (favorites_count >= 0)
);