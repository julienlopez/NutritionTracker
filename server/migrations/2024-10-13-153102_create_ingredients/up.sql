CREATE TABLE ingredients (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  comment TEXT NOT NULL,
  weight INTEGER NOT NULL,
  energy INTEGER NOT NULL,
  calories INTEGER NOT NULL,
  fats NUMERIC NOT NULL,
  saturated_fats NUMERIC NOT NULL,
  carbohydrates NUMERIC NOT NULL,
  sugars NUMERIC NOT NULL,
  fibers NUMERIC NOT NULL,
  proteins NUMERIC NOT NULL,
  salt NUMERIC NOT NULL
);