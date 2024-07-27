CREATE TABLE IF NOT EXISTS threads (
	id SERIAL PRIMARY KEY,
	thread_id VARCHAR(32) NOT NULL UNIQUE,
	
	title VARCHAR(128),
	queue VARCHAR(128),
	queue_index INT,
	
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'log_level') THEN
        EXECUTE 'CREATE TYPE log_level AS ENUM (''TRACE'', ''DEBUG'', ''INFO'', ''WARN'', ''ERROR'', ''FATAL'')';
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS logs (
    id SERIAL PRIMARY KEY,
    level log_level NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'alignment') THEN
        EXECUTE 'CREATE TYPE alignment AS ENUM (''Town'', ''Mafia'', ''Werewolf'', ''Cult'', ''SelfAlignedKilling'', ''SelfAlignedOther'', ''Unknown'')';
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS players (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    alignment alignment,
    role VARCHAR(128),
    has_won BOOLEAN,
    aliases TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    thread_id VARCHAR(32) NOT NULL,
    FOREIGN KEY (thread_id) REFERENCES threads(thread_id) ON DELETE CASCADE
);

