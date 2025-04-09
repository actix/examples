CREATE TABLE IF NOT EXISTS items
(
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR NOT NULL
);
