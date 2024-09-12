-- Add migration script here
-- Add migration script here


CREATE TABLE articles (
    id uuid NOT NULL,
    likes INT DEFAULT 0 NOT NULL,
    slug VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    summary VARCHAR(1000) NOT NULL,
    content TEXT NOT NULL,
    author_id uuid NOT NULL,
    status article_status DEFAULT 'draft' NOT NULL,
    thumbnail_uri VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ,

    PRIMARY KEY (id)
);