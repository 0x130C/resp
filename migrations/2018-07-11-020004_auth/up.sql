CREATE TABLE IF NOT EXISTS user_session (
    token VARCHAR NOT NULL PRIMARY KEY DEFAULT digest(gen_random_bytes(1024), 'sha512'),
    user_id uuid NOT NULL REFERENCES users(id) ON UPDATE CASCADE,
    expire TIMESTAMP NOT NULL DEFAULT (NOW() + '1 month'::interval)
)
