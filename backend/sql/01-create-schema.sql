-- Ingredients table
CREATE TABLE ingredients (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  quantity VARCHAR(50) NOT NULL
);

-- Recipes table
CREATE TABLE recipes (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  title TEXT NOT NULL,
  cid BIGINT DEFAULT 0,
  ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Recipe-Ingredient relationship table
CREATE TABLE recipe_ingredients (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  recipe_id BIGINT NOT NULL,
  ingredient_id BIGINT NOT NULL,
  ingredient_name varchar(255) DEFAULT 'unknown',
  quantity VARCHAR(50),
  cid BIGINT DEFAULT 0,
  ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Modification timestamp
  FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE, -- Reference to the recipes table
  FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE -- Reference to the ingredients table
);

-- Set the starting point for recipe and ingredient IDs (optional)
ALTER TABLE recipes AUTO_INCREMENT = 1000;
ALTER TABLE ingredients AUTO_INCREMENT = 1000;
ALTER TABLE recipe_ingredients AUTO_INCREMENT = 1000;
