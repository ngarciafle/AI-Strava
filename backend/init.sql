

CREATE TABLE IF NOT EXISTS runners (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS races (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    distance double precision NOT NULL,
    time double precision NOT NULL,
    times double precision[] NOT NULL,
    rithm double precision NOT NULL,
    rithms double precision[] NOT NULL,
    elevation_gain double precision NOT NULL,
    elevation_loss double precision NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    runner_id INT REFERENCES runners(id) NOT NULL
);
