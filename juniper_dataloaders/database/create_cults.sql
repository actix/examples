CREATE TABLE cults
(
	id serial NOT NULL,
	name VARCHAR(100) UNIQUE
);
 
-- ALTER TABLE CULTS owner TO jayylmao;

INSERT INTO cults (name) 
        VALUES ('Church of the Latter day dudes');
INSERT INTO cults (name) 
        VALUES ('Universal Medicine');
