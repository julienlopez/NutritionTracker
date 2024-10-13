CREATE TABLE dishes_to_ingredients (
    dish_id INTEGER NOT NULL REFERENCES dishes(id),
    ingredients_id INTEGER NOT NULL REFERENCES ingredients(id),
    quantity INTEGER NOT NULL,
    PRIMARY KEY(dish_id, ingredients_id)
);
