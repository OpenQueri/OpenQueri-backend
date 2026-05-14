-- UP
CREATE TABLE crawler_list (
    id SERIAL PRIMARY KEY,
    user_id INT,
    url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    owner_site TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
