CREATE TABLE meals(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO meals(id, name) VALUES(1, 'MORNING');
INSERT INTO meals(id, name) VALUES(2, 'LUNCH');
INSERT INTO meals(id, name) VALUES(3, 'AFTERNOON');
INSERT INTO meals(id, name) VALUES(4, 'DINNER');
INSERT INTO meals(id, name) VALUES(5, 'NIGHT');
