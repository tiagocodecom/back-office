-- Add migration script here
CREATE TYPE article_status AS ENUM ('draft', 'published', 'deleted');
