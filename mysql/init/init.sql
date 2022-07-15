DROP SCHEMA IF EXISTS rust_ddd_practice;
CREATE SCHEMA rust_ddd_practice;
USE rust_ddd_practice;

DROP TABLE IF EXISTS user;

CREATE TABLE user(
  id           INT(10),
  name     VARCHAR(40),
  age          INT(10),
--   0,1,2,9
  gender          INT(10),
);

INSERT INTO user (id, name, age, gender) VALUES (1, "Nagaoka", 18, 1);