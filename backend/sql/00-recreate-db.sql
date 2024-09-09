-- DEV Only - Comment out for keeping db between restarts
DROP DATABASE IF EXISTS cookbook;
DROP USER IF EXISTS 'bogdan'@'localhost';

-- DEV Only - for quick iteration
CREATE USER 'bogdan'@'localhost' IDENTIFIED BY 'macmacmac';
CREATE DATABASE cookbook CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
GRANT ALL PRIVILEGES ON cookbook.* TO 'bogdan'@'localhost';
FLUSH PRIVILEGES;
