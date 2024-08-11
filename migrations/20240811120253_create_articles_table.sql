-- Add migration script here
CREATE TABLE articles (
    id uuid NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author_id uuid NOT NULL REFERENCES authors(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);