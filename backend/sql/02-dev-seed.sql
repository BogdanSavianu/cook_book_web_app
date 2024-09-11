-- Dev seed for ingredients
INSERT INTO ingredients (name) VALUES ('tomatoes');

-- Dev seed for recipes
INSERT INTO recipes (title, cid) VALUES ('spaghetti', 123);

-- Dev seed for recipe_ingredients
INSERT INTO recipe_ingredients (recipe_id, ingredient_id, ingredient_name, quantity) VALUES (1000, 1000, 'tomatoes', '200 g');
