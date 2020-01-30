CREATE TABLE persons
(
id SERIAL NOT NULL,
name VARCHAR(100),
cult INTEGER
);
 
-- ALTER TABLE PERSONS owner TO jayylmao;

INSERT INTO persons (name, cult) 
        VALUES ('Tim', 1);
INSERT INTO persons (name, cult) 
        VALUES ('Bill', 1);
INSERT INTO persons (name, cult) 
        VALUES ('Bob', 2);
INSERT INTO persons (name, cult) 
        VALUES ('Hamo', 2);
INSERT INTO persons (name, cult) 
        VALUES ('Jerry', 1);
