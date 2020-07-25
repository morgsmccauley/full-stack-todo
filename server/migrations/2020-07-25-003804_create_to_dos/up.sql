CREATE TABLE to_dos (
  id VARCHAR NOT NULL PRIMARY KEY,
  label VARCHAR NOT NULL
);

INSERT INTO to_dos (id, label)
VALUES ('1', 'Groceries'), ('2', 'Cleaning'), ('3', 'Cooking')
