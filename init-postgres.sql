CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE accounts (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR ( 50 ) NOT NULL
);

INSERT INTO accounts VALUES (uuid_generate_v4(), 'David');
INSERT INTO accounts VALUES (uuid_generate_v4(), 'John');
INSERT INTO accounts VALUES (uuid_generate_v4(), 'Alan');

SELECT * FROM accounts;
