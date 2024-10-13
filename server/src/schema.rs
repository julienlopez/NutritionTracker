// @generated automatically by Diesel CLI.

diesel::table! {
    dishes (id) {
        id -> Int4,
        name -> Text,
        quantity -> Int4,
    }
}

diesel::table! {
    dishes_to_ingredients (dish_id, ingredients_id) {
        dish_id -> Int4,
        ingredients_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Int4,
        title -> Text,
        comment -> Text,
        weight -> Int4,
        energy -> Int4,
        calories -> Int4,
        fats -> Numeric,
        saturated_fats -> Numeric,
        carbohydrates -> Numeric,
        sugars -> Numeric,
        fibers -> Numeric,
        proteins -> Numeric,
        salt -> Numeric,
    }
}

diesel::table! {
    intakes (id) {
        id -> Int4,
        dish_id -> Int4,
        meal_id -> Int4,
        date -> Date,
    }
}

diesel::table! {
    meals (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(dishes_to_ingredients -> dishes (dish_id));
diesel::joinable!(dishes_to_ingredients -> ingredients (ingredients_id));
diesel::joinable!(intakes -> dishes (dish_id));
diesel::joinable!(intakes -> meals (meal_id));

diesel::allow_tables_to_appear_in_same_query!(
    dishes,
    dishes_to_ingredients,
    ingredients,
    intakes,
    meals,
);
