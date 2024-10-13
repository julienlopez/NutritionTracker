CREATE TABLE intakes(
    id SERIAL PRIMARY KEY,
    dish_id INTEGER NOT NULL REFERENCES dishes(id),
    meal_id INTEGER NOT NULL REFERENCES meals(id),
    date DATE NOT NULL DEFAULT now()
)
